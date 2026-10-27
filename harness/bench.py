#!/usr/bin/env python3
"""Harnais de benchmark : exécute chaque implémentation sur chaque workload, mesure
temps / CPU / RAM / disque de façon externe (wait4 + /proc) et vérifie la cohérence
des checksums entre implémentations. Python 3 stdlib uniquement.

  bench.py list
  bench.py run   --preset medium [--impls a,b] [--workloads w,..] [--repeat 3] [--timeout 900]
                 [--wrapper "taskset -c 0"] [--out results/run.json]
  bench.py report results/run.json          -> .md / .csv / .html à côté du JSON
"""
import argparse, csv, html, json, math, os, shlex, subprocess, sys, threading, time, statistics
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PAGE = os.sysconf("SC_PAGE_SIZE")
HZ = os.sysconf("SC_CLK_TCK")
BASE_WORKLOADS = ["sort", "sieve", "matmul", "lines", "hashmap"]
UNIT = {"sort": "éléments", "sieve": "entiers", "matmul": "mult-add", "lines": "lignes", "hashmap": "opérations"}
_BASE_WORK = {"sort": "n", "sieve": "n", "matmul": "n**3", "lines": "n", "hashmap": "3*n"}
_EXTRA = {}  # workloads « extrêmes » décrits par spec/workloads.d/<nom>.json : {"unit","work_items","ideal_bytes"} (expressions en n, t)
for _f in sorted((ROOT / "spec" / "workloads.d").glob("*.json")):
    _d = json.loads(_f.read_text())
    _EXTRA[_d["name"]] = _d
    UNIT[_d["name"]] = _d["unit"]
WORKLOADS = BASE_WORKLOADS + sorted(_EXTRA)


def _ev(expr, n, t):
    return eval(expr, {"__builtins__": {}}, {"n": n, "t": t, "math": math, "min": min, "max": max})


def work_items(w, n, t=1):
    return _ev(_EXTRA[w]["work_items"], n, t) if w in _EXTRA else _ev(_BASE_WORK[w], n, t)


def ideal_bytes(w, n, t=1):
    """Taille des structures de données allouées par la spécification (plancher mémoire)."""
    if w in _EXTRA: return _ev(_EXTRA[w]["ideal_bytes"], n, t)
    if w == "sort": return 8 * n
    if w == "sieve": return n + 1
    if w == "matmul": return 12 * n * n
    if w == "lines": return 15 * n + 16 * 2 ** 20
    if w == "hashmap":
        cap = 2
        while cap < 2 * n: cap *= 2
        return 16 * cap
    raise KeyError(w)


def load_presets():
    """presets.json (workloads de base) + harness/presets.d/<nom>.json : {"<workload>": {"smoke": n | {"n":..,"threads":..}, ...}}."""
    p = load("presets.json")
    for f in sorted((Path(__file__).parent / "presets.d").glob("*.json")):
        for w, by_preset in json.loads(f.read_text()).items():
            for name, v in by_preset.items():
                p.setdefault(name, {})[w] = v
    return p


def size_of(v):
    """Valeur de preset -> (n, threads|None)."""
    return (v["n"], v.get("threads")) if isinstance(v, dict) else (v, None)


def load_checksums():
    out = {}
    for p in [ROOT / "spec" / "checksums.json"] + sorted((ROOT / "spec" / "checksums.d").glob("*.json")):
        if p.exists(): out.update({k: v for k, v in json.loads(p.read_text()).items() if not k.startswith("_")})
    return out


def load(name):
    return json.loads((Path(__file__).parent / name).read_text())


def available(impls):
    out = {}
    for k, v in impls.items():
        exe = v["cmd"][0] if v["cmd"][0] != "dotnet" else v["cmd"][1]
        if (ROOT / exe).exists() or (v["cmd"][0] != "dotnet" and (ROOT / v["cmd"][0]).exists()):
            out[k] = v
    return out


def build_runner():
    """Compile harness/runner.c (fork/exec/wait4) si besoin ; renvoie son chemin."""
    exe = ROOT / "results" / ".runner"
    src = Path(__file__).parent / "runner.c"
    if not exe.exists() or exe.stat().st_mtime < src.stat().st_mtime:
        exe.parent.mkdir(exist_ok=True)
        subprocess.run(["gcc", "-O2", "-o", str(exe), str(src)], check=True)
    return str(exe)


def children_of(pid):
    try:
        return [int(x) for x in open(f"/proc/{pid}/task/{pid}/children").read().split()]
    except OSError:
        return []


def run_once(cmd, wrapper, timeout):
    runner = build_runner()
    out = ROOT / "results" / f".rusage.{os.getpid()}"
    argv = [runner, str(out)] + (shlex.split(wrapper) if wrapper else []) + cmd
    t0 = time.perf_counter()
    p = subprocess.Popen(argv, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    samples, stop, hwm = [], threading.Event(), [0]

    def sampler():
        child = None
        while not stop.is_set():
            try:
                if child is None:
                    kids = children_of(p.pid)
                    child = kids[0] if kids else None
                if child is not None:
                    with open(f"/proc/{child}/stat") as f:
                        parts = f.read().rsplit(")", 1)[1].split()
                    cpu = (int(parts[11]) + int(parts[12])) / HZ
                    with open(f"/proc/{child}/status") as f:
                        for ln in f:
                            if ln.startswith("VmHWM:"): hwm[0] = max(hwm[0], int(ln.split()[1]))
                            elif ln.startswith("VmRSS:"): rss = int(ln.split()[1])
                    samples.append((time.perf_counter() - t0, rss, cpu))
            except (FileNotFoundError, ProcessLookupError, IndexError, ValueError, NameError):
                pass
            stop.wait(0.02)

    th = threading.Thread(target=sampler, daemon=True)
    th.start()
    timed_out = False

    def killer():
        nonlocal timed_out
        timed_out = True
        for c in children_of(p.pid):
            try: os.kill(c, 9)
            except ProcessLookupError: pass

    timer = threading.Timer(timeout, killer)
    timer.start()
    stdout, stderr = p.communicate()
    wall = time.perf_counter() - t0
    timer.cancel(); stop.set(); th.join()
    try:
        f = [int(x) for x in out.read_text().split()]
        out.unlink()
    except (OSError, ValueError):
        f = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    _, status, ut, st, maxrss, minflt, majflt, inb, oub, vol, invol = f
    rc = os.waitstatus_to_exitcode(status) if status else 0
    rec = {"returncode": rc, "timed_out": timed_out, "wall_s": wall, "user_s": ut / 1e6, "sys_s": st / 1e6,
           "peak_rss_kb": max(maxrss, hwm[0]), "minflt": minflt, "majflt": majflt, "inblock": inb, "oublock": oub,
           "ctx_vol": vol, "ctx_invol": invol, "stderr": stderr.strip()[-300:]}
    for line in stdout.splitlines():
        if line.startswith("RESULT "):
            kv = dict(x.split("=", 1) for x in line[7:].split())
            rec["checksum"] = kv["checksum"]; rec["internal_s"] = int(kv["time_ns"]) / 1e9
    if len(samples) > 120:  # décimation de la courbe
        step = len(samples) / 120
        samples = [samples[int(i * step)] for i in range(120)]
    rec["timeline"] = [(round(t, 3), r, round(c, 3)) for t, r, c in samples]
    rec["status"] = ("timeout" if timed_out else "ok" if rc == 0 and "checksum" in rec
                     else "oom?" if rc in (-9, 137) else "failed")
    return rec


def cmd_run(a):
    impls = available(load("impls.json"))
    if a.impls: impls = {k: v for k, v in impls.items() if k in a.impls.split(",")}
    presets = load_presets()[a.preset]
    wl = a.workloads.split(",") if a.workloads else [w for w in WORKLOADS if w in presets]
    results = {"meta": {"preset": a.preset, "repeat": a.repeat, "wrapper": a.wrapper, "date": time.strftime("%F %T"),
                        "host": {"cpus": os.cpu_count(), "uname": " ".join(os.uname()[:3]),
                                 "mem_kb": int(open("/proc/meminfo").readline().split()[1])},
                        "sizes": {w: size_of(presets[w])[0] for w in wl}, "threads": {w: size_of(presets[w])[1] for w in wl if size_of(presets[w])[1]}}, "runs": []}
    if not impls: sys.exit("aucune implémentation construite (voir README : make all)")
    print(f"implémentations : {', '.join(impls)}", file=sys.stderr)
    smoke = load_presets()["smoke"]
    for name, spec in impls.items():
        # coût de démarrage : plus petit run, médiane de 3
        st = [run_once([c.format(workload="sort", n=smoke["sort"]) for c in spec["cmd"]], a.wrapper, 120) for _ in range(3)]
        results.setdefault("startup", {})[name] = {"wall_s": statistics.median(x["wall_s"] for x in st),
                                                  "rss_kb": statistics.median(x["peak_rss_kb"] for x in st)}
        for w in wl:
            n, thr = size_of(presets[w])
            if w in spec.get("unsupported", {}):
                results["runs"].append({"impl": name, "workload": w, "n": n, "threads": thr or 1, "rep": 0, "status": "unsupported",
                                        "stderr": spec["unsupported"][w], "returncode": 0, "timed_out": False, "wall_s": 0, "user_s": 0,
                                        "sys_s": 0, "peak_rss_kb": 0, "minflt": 0, "majflt": 0, "inblock": 0, "oublock": 0, "timeline": []})
                print(f"{name:14s} {w:8s} non implémenté : {spec['unsupported'][w]}", file=sys.stderr)
                continue
            for r in range(a.repeat):
                cmd = [c.format(workload=w, n=n) for c in spec["cmd"]] + ([str(thr)] if thr else [])
                rec = run_once(cmd, a.wrapper, a.timeout)
                rec.update(impl=name, workload=w, n=n, threads=thr or 1, rep=r)
                results["runs"].append(rec)
                print(f"{name:14s} {w:8s} n={n:<11d} #{r} {rec['status']:7s} wall={rec['wall_s']:7.2f}s "
                      f"cpu={rec['user_s']+rec['sys_s']:7.2f}s rss={rec['peak_rss_kb']/1024:8.1f}MB", file=sys.stderr)
                if rec["status"] != "ok": break  # inutile de répéter un échec
    out = Path(a.out or ROOT / "results" / f"run-{a.preset}-{time.strftime('%Y%m%d-%H%M%S')}.json")
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(results))
    print(out)
    report(out)


# ----------------------------------------------------------------------------- rapport

def gmean(xs):
    xs = [x for x in xs if x and x > 0]
    return math.exp(sum(math.log(x) for x in xs) / len(xs)) if xs else float("nan")


def summarize(data):
    """-> {(workload): {impl: dict}} avec médianes, et vérification des checksums."""
    impls_meta = load("impls.json")
    by = {}
    for r in data["runs"]:
        by.setdefault((r["workload"], r["impl"]), []).append(r)
    table = {}
    for (w, impl), rs in by.items():
        ok = [r for r in rs if r["status"] == "ok"]
        n = rs[0]["n"]; t = rs[0].get("threads", 1)
        if not ok:
            table.setdefault(w, {})[impl] = {"status": rs[-1]["status"], "n": n, "stderr": rs[-1]["stderr"]}
            continue
        med = lambda k: statistics.median(r[k] for r in ok)
        best = min(ok, key=lambda r: r["internal_s"])
        cpu = med("user_s") + med("sys_s")
        table.setdefault(w, {})[impl] = {
            "status": "ok", "n": n, "threads": t, "internal_s": med("internal_s"), "wall_s": med("wall_s"),
            "cpu_s": cpu, "user_s": med("user_s"), "sys_s": med("sys_s"),
            "cpu_pct": 100 * cpu / med("wall_s"), "rss_mb": med("peak_rss_kb") / 1024,
            "rss_ratio": med("peak_rss_kb") * 1024 / ideal_bytes(w, n, t),
            "minflt": med("minflt"), "majflt": med("majflt"), "inblock": med("inblock"), "oublock": med("oublock"),
            "throughput": work_items(w, n, t) / med("internal_s"),
            "mb_s_per_rss": work_items(w, n, t) / med("internal_s") / (med("peak_rss_kb") / 1024),
            "checksum": ok[0]["checksum"], "consistent": len({r["checksum"] for r in ok}) == 1,
            "internal_spread": (max(r["internal_s"] for r in ok) - min(r["internal_s"] for r in ok)) / med("internal_s"),
            "timeline": best["timeline"], "runs": len(ok)}
    # cohérence inter-implémentations
    for w, d in table.items():
        sums = {v["checksum"] for v in d.values() if v.get("status") == "ok"}
        for v in d.values():
            if v.get("status") == "ok": v["agrees"] = len(sums) == 1
        d["_checksums"] = sorted(sums)
        # comparaison à la table de référence (spec/checksums.json) quand la taille y figure
        n = next(v["n"] for v in d.values() if isinstance(v, dict) and "n" in v)
        ref = load_checksums().get(f"{w}:{n}")
        for v in d.values():
            if isinstance(v, dict) and v.get("status") == "ok" and ref:
                v["agrees"] = v["agrees"] and v["checksum"] == ref
    return table, impls_meta


def fmt_rate(x):
    for unit, f in (("G", 1e9), ("M", 1e6), ("k", 1e3)):
        if x >= f: return f"{x/f:.2f} {unit}"
    return f"{x:.0f} "


def report(path):
    path = Path(path)
    data = json.loads(path.read_text())
    table, meta = summarize(data)
    fp = ROOT / "results" / "footprint.json"
    footprint = json.loads(fp.read_text()) if fp.exists() else {}
    md = [f"# Résultats — preset `{data['meta']['preset']}`", "",
          f"_{data['meta']['date']} · {data['meta']['host']['cpus']} CPU · {data['meta']['host']['mem_kb']//1024} Mo RAM · "
          f"{data['meta']['host']['uname']} · médiane de {data['meta']['repeat']} exécutions_", ""]
    agg_t, agg_m, agg_c = {}, {}, {}
    for w in WORKLOADS:
        if w not in table: continue
        d = {k: v for k, v in table[w].items() if not k.startswith("_")}
        oks = {k: v for k, v in d.items() if v["status"] == "ok"}
        n = next(iter(d.values()))["n"]
        md += [f"## {w} (n = {n:,})".replace(",", " "), "",
               f"Débit = {UNIT[w]}/s mesuré à l'intérieur du processus (hors démarrage du runtime). "
               f"RAM/idéal = pic RSS ÷ taille des structures de la spécification ({ideal_bytes(w, n, next(iter(d.values())).get("threads", 1))/2**20:.0f} Mo).", "",
               "| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |",
               "|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|"]
        best_t = min((v["internal_s"] for v in oks.values()), default=None)
        best_c = min((v["cpu_s"] for v in oks.values()), default=None)
        best_m = min((v["rss_mb"] for v in oks.values()), default=None)
        for k, v in sorted(d.items(), key=lambda kv: kv[1].get("internal_s", 1e99)):
            lab = meta.get(k, {}).get("label", k)
            if v["status"] != "ok":
                md.append(f"| {lab} | **{v['status']}** | | | | | | | | | |"); continue
            agg_t.setdefault(k, []).append(v["internal_s"] / best_t)
            agg_c.setdefault(k, []).append(v["cpu_s"] / best_c)
            agg_m.setdefault(k, []).append(v["rss_mb"] / best_m)
            md.append(f"| {lab} | {fmt_rate(v['throughput'])}/s | {v['internal_s']:.3f} s | ×{v['internal_s']/best_t:.2f} | "
                      f"{v['cpu_s']:.2f} s | {v['cpu_pct']:.0f} % | {v['rss_mb']:.0f} Mo | ×{v['rss_ratio']:.2f} | "
                      f"{v['minflt']:.0f} / {v['majflt']:.0f} | {v['inblock']:.0f} / {v['oublock']:.0f} | "
                      f"{'✅' if v.get('agrees') else '❌ ' + v['checksum']} |")
        md.append("")
    if agg_t:
        md += ["## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)", "",
               "| implémentation | temps | CPU | pic RAM | workloads |", "|---|--:|--:|--:|--:|"]
        for k in sorted(agg_t, key=lambda k: gmean(agg_t[k])):
            md.append(f"| {meta.get(k, {}).get('label', k)} | ×{gmean(agg_t[k]):.2f} | ×{gmean(agg_c[k]):.2f} | ×{gmean(agg_m[k]):.2f} | {len(agg_t[k])} |")
        md.append("")
    if data.get("startup"):
        md += ["## Coût de démarrage (run minuscule, médiane de 3)", "", "| implémentation | durée | RAM |", "|---|--:|--:|"]
        for k, v in data["startup"].items():
            md.append(f"| {meta.get(k, {}).get('label', k)} | {v['wall_s']*1000:.0f} ms | {v['rss_kb']/1024:.1f} Mo |")
        md.append("")
    if footprint:
        md += ["## Empreinte disque et coût de construction", "",
               "| implémentation | exécutable | runtime/deps requis | compilation | vérification |", "|---|--:|--:|--:|--:|"]
        for k, v in footprint.items():
            md.append(f"| {meta.get(k, {}).get('label', k)} | {v.get('binary_kb', 0)/1024:.2f} Mo | {v.get('runtime_kb', 0)/1024:.1f} Mo | "
                      f"{v.get('build_s', 0):.1f} s | {('%.1f s' % v['verify_s']) if v.get('verify_s') is not None else '—'} |")
        md.append("")
    path.with_suffix(".md").write_text("\n".join(md))
    with open(path.with_suffix(".csv"), "w", newline="") as f:
        wr = csv.writer(f)
        cols = ["workload", "impl", "n", "status", "internal_s", "wall_s", "cpu_s", "cpu_pct", "rss_mb", "rss_ratio", "throughput",
                "minflt", "majflt", "inblock", "oublock", "checksum", "agrees"]
        wr.writerow(cols)
        for w, d in table.items():
            for k, v in d.items():
                if not k.startswith("_"): wr.writerow([w, k, v["n"], v["status"]] + [v.get(c, "") for c in cols[4:]])
    write_html(path.with_suffix(".html"), data, table, meta, agg_t, agg_c, agg_m, footprint)
    print(f"rapport : {path.with_suffix('.md')}  {path.with_suffix('.html')}  {path.with_suffix('.csv')}", file=sys.stderr)


def spark(tl, w=160, h=34):
    if len(tl) < 2: return ""
    tmax = max(t for t, _, _ in tl) or 1; rmax = max(r for _, r, _ in tl) or 1
    pts = " ".join(f"{t/tmax*w:.1f},{h-2-(r/rmax)*(h-4):.1f}" for t, r, _ in tl)
    return f'<svg class="spark" viewBox="0 0 {w} {h}" width="{w}" height="{h}"><polyline points="{pts}" fill="none" stroke="var(--accent)" stroke-width="1.5"/></svg>'


def write_html(path, data, table, meta, agg_t, agg_c, agg_m, footprint):
    e = html.escape
    out = ["""<!doctype html><html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Benchmark programmation vérifiée</title><style>
:root{--bg:#fafaf7;--fg:#1d1d1b;--mut:#6b6b66;--line:#e2e1da;--accent:#2f6fdb;--bar:#cfdcf5;--bad:#c0392b;--good:#1e8449;--card:#fff}
@media (prefers-color-scheme:dark){:root{--bg:#161715;--fg:#ecece8;--mut:#9a9a93;--line:#2c2d2a;--accent:#6ea2ff;--bar:#27395c;--bad:#ff7b6b;--good:#5fd08a;--card:#1d1e1c}}
body{background:var(--bg);color:var(--fg);font:15px/1.5 system-ui,sans-serif;margin:0;padding:24px 16px}main{max-width:1180px;margin:auto}
h1{font-size:24px}h2{font-size:18px;margin-top:36px}p.meta{color:var(--mut)}
table{border-collapse:collapse;width:100%;background:var(--card);border:1px solid var(--line);border-radius:8px;overflow:hidden;font-size:13.5px}
th,td{padding:6px 10px;border-bottom:1px solid var(--line);text-align:right;white-space:nowrap}th:first-child,td:first-child{text-align:left}
th{color:var(--mut);font-weight:600;font-size:12px}.wrap{overflow-x:auto}
td.bar{position:relative}td.bar span{position:relative;z-index:1}td.bar i{position:absolute;left:0;top:0;bottom:0;background:var(--bar);z-index:0}
.bad{color:var(--bad);font-weight:600}.good{color:var(--good)}.spark{vertical-align:middle}
</style></head><body><main><h1>Benchmark : programmation vérifiée (F*, Creusot) vs références</h1>"""]
    m = data["meta"]
    out.append(f'<p class="meta">preset <b>{e(m["preset"])}</b> · {e(m["date"])} · {m["host"]["cpus"]} CPU · {m["host"]["mem_kb"]//1024} Mo RAM · médiane de {m["repeat"]} exécutions</p>')

    def bar(val, mx, text):
        pct = 0 if not mx else min(100, 100 * val / mx)
        return f'<td class="bar"><i style="width:{pct:.0f}%"></i><span>{text}</span></td>'

    for w in WORKLOADS:
        if w not in table: continue
        d = {k: v for k, v in table[w].items() if not k.startswith("_")}
        oks = [v for v in d.values() if v["status"] == "ok"]
        n = next(iter(d.values()))["n"]
        out.append(f'<h2>{w} — n = {n:,}</h2><div class="wrap"><table><tr><th>implémentation</th><th>débit ({UNIT[w]}/s)</th><th>temps</th><th>CPU user+sys</th><th>pic RAM</th><th>RAM/idéal</th><th>courbe RAM</th><th>fautes page</th><th>E/S blocs</th><th>checksum</th></tr>'.replace(",", " "))
        mt = max((v["throughput"] for v in oks), default=1); mc = max((v["cpu_s"] for v in oks), default=1); mr = max((v["rss_mb"] for v in oks), default=1)
        for k, v in sorted(d.items(), key=lambda kv: kv[1].get("internal_s", 1e99)):
            lab = e(meta.get(k, {}).get("label", k))
            if v["status"] != "ok":
                out.append(f'<tr><td>{lab}</td><td colspan="9" class="bad">{e(v["status"])} {e(v.get("stderr",""))}</td></tr>'); continue
            out.append(f'<tr><td>{lab}</td>{bar(v["throughput"], mt, fmt_rate(v["throughput"])+"/s")}<td>{v["internal_s"]:.3f} s</td>'
                       f'{bar(v["cpu_s"], mc, "%.2f s" % v["cpu_s"])}{bar(v["rss_mb"], mr, "%.0f Mo" % v["rss_mb"])}<td>×{v["rss_ratio"]:.2f}</td>'
                       f'<td>{spark(v["timeline"])}</td><td>{v["minflt"]:.0f}/{v["majflt"]:.0f}</td><td>{v["inblock"]:.0f}/{v["oublock"]:.0f}</td>'
                       f'<td class="{"good" if v.get("agrees") else "bad"}">{"✓" if v.get("agrees") else "✗"}</td></tr>')
        out.append("</table></div>")
    if agg_t:
        out.append('<h2>Synthèse (moyenne géométrique ; 1.00 = meilleur)</h2><div class="wrap"><table><tr><th>implémentation</th><th>temps</th><th>CPU</th><th>pic RAM</th></tr>')
        for k in sorted(agg_t, key=lambda k: gmean(agg_t[k])):
            out.append(f'<tr><td>{e(meta.get(k, {}).get("label", k))}</td><td>×{gmean(agg_t[k]):.2f}</td><td>×{gmean(agg_c[k]):.2f}</td><td>×{gmean(agg_m[k]):.2f}</td></tr>')
        out.append("</table></div>")
    if footprint:
        out.append('<h2>Empreinte disque et coût de construction</h2><div class="wrap"><table><tr><th>implémentation</th><th>exécutable</th><th>runtime requis</th><th>compilation</th><th>vérification</th></tr>')
        for k, v in footprint.items():
            vs = ("%.1f s" % v["verify_s"]) if v.get("verify_s") is not None else "—"
            out.append(f'<tr><td>{e(meta.get(k, {}).get("label", k))}</td><td>{v.get("binary_kb",0)/1024:.2f} Mo</td><td>{v.get("runtime_kb",0)/1024:.1f} Mo</td><td>{v.get("build_s",0):.1f} s</td><td>{vs}</td></tr>')
        out.append("</table></div>")
    out.append("</main></body></html>")
    path.write_text("\n".join(out))


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sp = ap.add_subparsers(dest="cmd", required=True)
    sp.add_parser("list")
    r = sp.add_parser("run")
    r.add_argument("--preset", default="medium"); r.add_argument("--impls"); r.add_argument("--workloads")
    r.add_argument("--repeat", type=int, default=3); r.add_argument("--timeout", type=int, default=900)
    r.add_argument("--wrapper", default=""); r.add_argument("--out")
    rp = sp.add_parser("report"); rp.add_argument("file")
    a = ap.parse_args()
    if a.cmd == "list":
        av = available(load("impls.json"))
        for k, v in load("impls.json").items(): print(f"{'✔' if k in av else '✘'} {k:14s} {v['label']}")
        for p, s in load("presets.json").items():
            if not p.startswith("_"): print(p, s)
    elif a.cmd == "run": cmd_run(a)
    else: report(a.file)


if __name__ == "__main__":
    main()
