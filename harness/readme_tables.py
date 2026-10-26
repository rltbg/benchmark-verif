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
    files = [R / f"run-{preset}.json"] + [R / f"run-{preset}-{i}.json" for i in ("fstar-ocaml", "fstar-ocaml-fast")]
    data = None
    for f in files:
        if not f.exists(): continue
        d = json.loads(f.read_text())
        if data is None: data = d
        else: data["runs"] += d["runs"]
    return data

def cell(v):
    if v["status"] != "ok":
        return {"timeout": "délai dépassé", "failed": "**échec**"}.get(v["status"], v["status"])
    return f"{bench.fmt_rate(v['throughput'])}/s · {v['internal_s']:.2f} s · {v['rss_mb']:.0f} Mo" + ("" if v.get("agrees") else " ❌")

def main():
    for preset in ["xlarge", "large", "medium", "small", "tiny"]:
        data = merged(preset)
        if not data: continue
        table, meta = bench.summarize(data)
        impls = [i for i in ORDER if any(i in table[w] for w in table)]
        print(f"### `{preset}`\n")
        print("| workload (n) | " + " | ".join(SHORT[i] for i in impls) + " |")
        print("|---|" + "--:|" * len(impls))
        for w in bench.WORKLOADS:
            if w not in table: continue
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
            f"{SHORT[k]} ×{bench.gmean(agg[k]):.2f} ({len(agg[k])}/5)" for k in sorted(agg, key=lambda k: bench.gmean(agg[k]))) + ".\n")

if __name__ == "__main__":
    main()
