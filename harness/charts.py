#!/usr/bin/env python3
"""Graphiques du README, générés depuis results/run-*.json -> results/charts/*.png (nécessite matplotlib)."""
import sys, math
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent))
import bench, readme_tables as rt
ORDER = [i for i in rt.ORDER if "ocaml" not in i]  # OCaml exclu des graphes : 25 à 1000x plus lent, il écrase les échelles (voir tableau du README)
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt, matplotlib.ticker

OUT = rt.R / "charts"; OUT.mkdir(exist_ok=True)
COL = {"c-native": "#6b7280", "zig": "#f59e0b", "rust-plain": "#b45309", "rust-creusot": "#dc2626",
       "fstar-c": "#2563eb", "fstar-fsharp": "#7c3aed", "fstar-ocaml-fast": "#059669", "fstar-ocaml": "#065f46"}
W = bench.BASE_WORKLOADS  # les graphes du README portent sur les 5 workloads de base

def tables(preset):
    d = rt.merged(preset)
    return bench.summarize(d)[0] if d else None

def ok(v): return isinstance(v, dict) and v.get("status") == "ok"

def relative_bars(preset, impls, fname, title):
    t = tables(preset); fig, ax = plt.subplots(figsize=(11, 4.6))
    n = len(impls); bw = 0.8 / n
    for j, i in enumerate(impls):
        ys = []
        for w in W:
            oks = [v["internal_s"] for v in t[w].values() if ok(v)]
            v = t[w].get(i); ys.append(v["internal_s"] / min(oks) if ok(v) else float("nan"))
        ax.bar([k + j * bw for k in range(len(W))], ys, bw, label=rt.SHORT[i], color=COL[i])
    ax.axhline(1, color="k", lw=.6); ax.set_xticks([k + .4 - bw / 2 for k in range(len(W))]); ax.set_xticklabels(W)
    ax.set_yscale("log"); ax.set_ylabel("temps relatif au meilleur (1 = meilleur, échelle log)"); ax.set_title(title); ax.legend(ncol=3, fontsize=8)
    ax.grid(axis="y", alpha=.3); fig.tight_layout(); fig.savefig(OUT / fname, dpi=130); plt.close(fig)

def scaling(fname):
    fig, axes = plt.subplots(1, 5, figsize=(16, 3.6))
    for ax, w in zip(axes, W):
        for i in ORDER:
            xs, ys = [], []
            for p in ["tiny", "small", "medium", "large", "xlarge"]:
                t = tables(p)
                if t and w in t and ok(t[w].get(i)): xs.append(t[w][i]["n"]); ys.append(t[w][i]["throughput"])
            if xs: ax.plot(xs, ys, "o-", ms=3, label=rt.SHORT[i], color=COL[i])
        ax.set_xscale("log"); ax.set_yscale("log"); ax.xaxis.set_minor_formatter(matplotlib.ticker.NullFormatter()); ax.set_title(w); ax.set_xlabel("n"); ax.grid(alpha=.3)
    axes[0].set_ylabel("débit (éléments/s, log)"); axes[0].legend(fontsize=6)
    fig.suptitle("Débit en fonction de la taille (échelle log) — F*→OCaml exclu"); fig.tight_layout()
    fig.savefig(OUT / fname, dpi=130); plt.close(fig)

def memory(fname):
    t = tables("large"); fig, ax = plt.subplots(figsize=(11, 4.4)); impls = [i for i in ORDER if any(ok(t[w].get(i)) for w in W)]
    bw = 0.8 / len(impls)
    for j, i in enumerate(impls):
        ax.bar([k + j * bw for k in range(len(W))], [t[w][i]["rss_ratio"] if ok(t[w].get(i)) else float("nan") for w in W], bw, label=rt.SHORT[i], color=COL[i])
    ax.axhline(1, color="k", lw=.6); ax.set_yscale("log"); ax.set_xticks([k + .4 - bw / 2 for k in range(len(W))]); ax.set_xticklabels(W)
    ax.set_ylabel("pic RAM / taille des structures (log)"); ax.set_title("Empreinte mémoire à `large` (1 = idéal) — F*→OCaml exclu"); ax.legend(ncol=3, fontsize=8)
    ax.grid(axis="y", alpha=.3); fig.tight_layout(); fig.savefig(OUT / fname, dpi=130); plt.close(fig)

def proof_cost(fname):
    import json
    f = json.loads((rt.R / "footprint.json").read_text()); fig, ax = plt.subplots(figsize=(6, 3.2))
    names = ["Creusot (36 fichiers)", "F* (9 modules)"]; vals = [f["rust-creusot"]["verify_s"], f["fstar-c"]["verify_s"]]
    ax.barh(names, vals, color=[COL["rust-creusot"], COL["fstar-c"]]); ax.set_xlabel("secondes, à froid")
    for k, v in enumerate(vals): ax.text(v, k, f" {v:.0f} s", va="center")
    ax.set_title("Coût de la vérification"); fig.tight_layout(); fig.savefig(OUT / fname, dpi=130); plt.close(fig)

def ext_relative(preset, fname, title):
    """Temps relatif au meilleur pour TOUS les workloads (base + étendus) d'une taille, F*→OCaml exclu."""
    t = tables(preset); ws = bench.report_order(t); impls = [i for i in native if any(i in t[w] for w in ws)]
    fig, ax = plt.subplots(figsize=(15, 5)); bw = 0.8 / len(impls)
    for j, i in enumerate(impls):
        ys = []
        for w in ws:
            oks = [v["internal_s"] for v in t[w].values() if ok(v)]
            v = t[w].get(i); ys.append(v["internal_s"] / min(oks) if ok(v) and oks else float("nan"))
        ax.bar([k + j * bw for k in range(len(ws))], ys, bw, label=rt.SHORT[i], color=COL[i])
    ax.axhline(1, color="k", lw=.6); ax.set_yscale("log"); ax.set_xticks([k + .4 - bw / 2 for k in range(len(ws))]); ax.set_xticklabels(ws, rotation=30, ha="right")
    ax.set_ylabel("temps relatif au meilleur (log)"); ax.set_title(title); ax.legend(ncol=3, fontsize=8); ax.grid(axis="y", alpha=.3)
    fig.tight_layout(); fig.savefig(OUT / fname, dpi=130); plt.close(fig)

def memory_saturate(fname):
    t = tables("saturate"); ws = bench.report_order(t); impls = [i for i in native if any(i in t[w] for w in ws)]
    fig, ax = plt.subplots(figsize=(15, 4.6)); bw = 0.8 / len(impls)
    for j, i in enumerate(impls):
        ax.bar([k + j * bw for k in range(len(ws))], [t[w][i]["rss_mb"] / 1024 if ok(t[w].get(i)) else float("nan") for w in ws], bw, label=rt.SHORT[i], color=COL[i])
    ax.set_yscale("log"); ax.set_xticks([k + .4 - bw / 2 for k in range(len(ws))]); ax.set_xticklabels(ws, rotation=30, ha="right")
    ax.set_ylabel("pic RAM (Go, log)"); ax.set_title("Pic de RAM à `saturate` (15 Go disponibles)"); ax.legend(ncol=3, fontsize=8); ax.grid(axis="y", alpha=.3)
    fig.tight_layout(); fig.savefig(OUT / fname, dpi=130); plt.close(fig)

def scaling_chart(fname):
    import json
    f = rt.R / "run-large-scaling.json"; t = bench.summarize(json.loads(f.read_text()))[0]
    fig, axes = plt.subplots(1, 3, figsize=(13, 3.8))
    for ax, base in zip(axes, ("parmatmul", "parsort", "parsha")):
        for i in native:
            xs, ys = [], []
            v1 = t.get(f"{base}@1", {}).get(i)
            for T in (1, 2, 4):
                v = t.get(f"{base}@{T}", {}).get(i)
                if ok(v) and ok(v1): xs.append(T); ys.append(v1["internal_s"] / v["internal_s"])
            if xs: ax.plot(xs, ys, "o-", label=rt.SHORT[i], color=COL[i])
        ax.plot([1, 2, 4], [1, 2, 4], "k:", lw=.8, label="idéal"); ax.set_xticks([1, 2, 4]); ax.set_title(base); ax.set_xlabel("threads"); ax.grid(alpha=.3)
    axes[0].set_ylabel("accélération vs 1 thread"); axes[0].legend(fontsize=7); fig.tight_layout(); fig.savefig(OUT / fname, dpi=130); plt.close(fig)

native = ["c-native", "zig", "rust-plain", "rust-creusot", "fstar-c", "fstar-fsharp"]
relative_bars("large", native, "relatif-large.png", "Temps relatif au meilleur, `large` (F*→OCaml exclu, voir tableau)")
scaling("debit-vs-taille.png")
for _p in ("large", "xlarge", "saturate"):
    try: ext_relative(_p, f"tous-{_p}.png", f"Temps relatif au meilleur, `{_p}`, tous les workloads (F*→OCaml exclu)")
    except Exception as e: print("ext_relative", _p, e)
try: memory_saturate("memoire-saturate.png"); scaling_chart("passage-echelle.png")
except Exception as e: print("charts", e); memory("memoire-large.png"); proof_cost("cout-preuve.png")
print("ok", sorted(p.name for p in OUT.iterdir()))
