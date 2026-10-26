PRESET ?= medium
REPEAT ?= 3

.PHONY: build fstar-verify creusot bench list clean

# Construit Rust et les extractions F* (C, F#, OCaml) ; écrit results/footprint.json.
build:
	scripts/build-all.sh

fstar-verify:
	fstar/build.sh verify

# Preuves Creusot du crate Rust (nécessite cargo-creusot : scripts/setup-creusot.sh).
creusot:
	scripts/prove-creusot.sh

list:
	python3 harness/bench.py list

bench:
	python3 harness/bench.py run --preset $(PRESET) --repeat $(REPEAT)

clean:
	rm -rf impls/rust/target fstar/_build results/.runner
