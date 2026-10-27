#!/usr/bin/env python3
"""Génère les tableaux de résultats du README (Markdown) à partir de results/run-*.json.
Usage : python3 harness/readme_tables.py  > results/readme-tables.md"""
import json, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent))
import bench

R = Path(__file__).parent.parent / "results"
ORDER = ["c-native", "zig", "rust-plain", "rust-creusot", "fstar-c", "fstar-fsharp", "fstar-ocaml-fast", "fstar-ocaml"]
SHORT = {"c-native": "C natif", "zig": "Zig 0.17", "rust-plain": "Rust", "rust-creusot": "Rust + Creusot",
         "fstar-c": "F*→C", "fstar-fsharp": "F*→F#", "fstar-ocaml-fast": "F*→OCaml (corrigé)", "fstar-ocaml": "F*→OCaml (livré)"}

def merged(preset):
    """Fusionne run-<preset>.json et les campagnes OCaml run-<preset>-<impl>.json."""
    files = [R / f"run-{preset}.json"] + [R / f"run-{preset}-{i}.json" for i in ("ext", "par", "fstar-ocaml", "fstar-ocaml-fast", "zig")]
    data = None
    for f in files:
        if not f.exists(): continue
        d = json.loads(f.read_text())
        if f.name.endswith("-zig.json") and data:  # relance de Zig après optimisation de matmul : remplace les anciennes mesures de base
            data["runs"] = [r for r in data["runs"] if r["impl"] != "zig" or r["workload"] not in bench.BASE_WORKLOADS]
        if data is None: data = d
        else: data["runs"] += d["runs"]
    return data

def cell(v):
    if v["status"] != "ok":
        return {"timeout": "délai dépassé", "failed": "**échec**"}.get(v["status"], v["status"])
    return f"{bench.fmt_rate(v['throughput'])}/s · {v['internal_s']:.2f} s · {v['rss_mb']:.0f} Mo" + ("" if v.get("agrees") else " ❌")

def main():
    for preset in ["saturate", "xlarge", "large", "medium", "small", "tiny"]:
        data = merged(preset)
        if not data: continue
        table, meta = bench.summarize(data)
        impls = [i for i in ORDER if any(i in table[w] for w in table)]
        print(f"### `{preset}`\n")
        print("| workload (n) | " + " | ".join(SHORT[i] for i in impls) + " |")
        print("|---|" + "--:|" * len(impls))
        for w in bench.report_order(table):
            d = table[w]; n = next(v["n"] for k, v in d.items() if isinstance(v, dict) and "n" in v)
            print(f"| {w} ({n:,}) | ".replace(",", " ") + " | ".join(cell(d[i]) if i in d else "—" for i in impls) + " |")
        # synthèse (moyenne géométrique du temps relatif au meilleur) sur les workloads complets
        agg = {}
        for w in table:
            oks = {k: v for k, v in table[w].items() if isinstance(v, dict) and v.get("status") == "ok"}
            if not oks: continue
            bt = min(v["internal_s"] for v in oks.values())
            for k, v in oks.items(): agg.setdefault(k, []).append(v["internal_s"] / bt)
        print("\nMoyenne géométrique du temps relatif au meilleur : " + ", ".join(
            f"{SHORT[k]} ×{bench.gmean(agg[k]):.2f} ({len(agg[k])}/{len(table)})" for k in sorted(agg, key=lambda k: bench.gmean(agg[k]))) + ".\n")

def footprint(preset="large"):
    """Tableaux CPU/RAM par workload et empreinte disque/démarrage/construction (cités dans le README)."""
    data = merged(preset); table, meta = bench.summarize(data)
    impls = [i for i in ORDER if any(i in table[w] for w in table)]
    out = [f"#### CPU et RAM, `{preset}`\n",
           "Cellule : temps CPU (user+sys) · utilisation d'un cœur · pic RAM (RSS) · RAM ÷ taille idéale des structures · fautes de page mineures.\n",
           "| workload | " + " | ".join(SHORT[i] for i in impls) + " |", "|---|" + "--:|" * len(impls)]
    for w in bench.report_order(table):
        cells = []
        for i in impls:
            v = table[w].get(i)
            cells.append(f"{v['cpu_s']:.1f} s · {v['cpu_pct']:.0f} % · {v['rss_mb']:.0f} Mo · ×{v['rss_ratio']:.2f} · {v['minflt']/1e3:.0f} k" if v and v.get("status") == "ok" else "—")
        out.append(f"| {w} | " + " | ".join(cells) + " |")
    fp = json.loads((R / "footprint.json").read_text()); st = data.get("startup", {})
    out += ["", "#### Empreinte disque, démarrage, construction\n",
            "| colonne | exécutable | runtime à installer | démarrage (run minuscule) | RAM au démarrage | compilation | vérification à froid |", "|---|--:|--:|--:|--:|--:|--:|"]
    for i in impls:
        f = fp.get(i, {}); s_ = st.get(i, {})
        out.append(f"| {SHORT[i]} | {f.get('binary_kb', 0):.0f} Ko | {f.get('runtime_kb', 0)/1024:.0f} Mo | " +
                   (f"{s_['wall_s']*1000:.0f} ms | {s_['rss_kb']/1024:.1f} Mo | " if s_ else "— | — | ") + f"{f.get('build_s', 0):.1f} s | "
                   + (f"{f['verify_s']:.0f} s" if f.get("verify_s") else "—") + " |")
    print("\n".join(out))

def scaling(preset="large"):
    """Passage à l'échelle des workloads parallèles : temps et accélération à 1, 2, 4 threads (run-<preset>-scaling.json)."""
    f = R / f"run-{preset}-scaling.json"
    if not f.exists(): return
    table, _ = bench.summarize(json.loads(f.read_text()))
    impls = [i for i in ORDER if any(i in table[w] for w in table)]
    print(f"#### Passage à l'échelle (taille `{preset}`) : temps · accélération vs 1 thread\n")
    print("| workload | threads | " + " | ".join(SHORT[i] for i in impls) + " |"); print("|---|--:|" + "--:|" * len(impls))
    for base in ("parmatmul", "parsort", "parsha"):
        for t in (1, 2, 4):
            row = []
            for i in impls:
                v = table.get(f"{base}@{t}", {}).get(i); v1 = table.get(f"{base}@1", {}).get(i)
                row.append(f"{v['internal_s']:.1f} s · ×{v1['internal_s']/v['internal_s']:.2f}" if v and v.get("status") == "ok" and v1 and v1.get("status") == "ok" else "—")
            print(f"| {base} | {t} | " + " | ".join(row) + " |")
    print()

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "footprint": footprint(*sys.argv[2:3])
    elif len(sys.argv) > 1 and sys.argv[1] == "scaling": scaling(*sys.argv[2:3])
    else: main()
