PRESET ?= medium
REPEAT ?= 3

.PHONY: refs fstar fstar-verify creusot bench report list clean all

# Implémentations de référence (C, Rust, OCaml, F#) : construisables avec gcc, cargo, ocaml, dotnet.
refs:
	scripts/build-refs.sh

# Pipeline F* complet (nécessite F* + KaRaMeL ; voir README et scripts/setup-*.sh).
fstar-verify:
	fstar/build.sh verify
fstar:
	fstar/build.sh all

# Preuve Creusot du crate Rust (nécessite cargo-creusot + why3 + solveurs ; voir scripts/setup-creusot.sh).
creusot:
	scripts/prove-creusot.sh

list:
	python3 harness/bench.py list

bench:
	python3 harness/bench.py run --preset $(PRESET) --repeat $(REPEAT)

report:
	@test -n "$(FILE)" || (echo "make report FILE=results/run-....json"; exit 2)
	python3 harness/bench.py report $(FILE)

all: refs bench

clean:
	rm -rf impls/rust/target impls/fsharp-ref/{bin,obj,out} fstar/_build impls/c-ref/bench impls/ocaml-ref/bench
