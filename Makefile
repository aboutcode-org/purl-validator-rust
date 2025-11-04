build-fst:
	cargo run --bin fst_builder

build-python:
	maturin build --release

clean:
	cargo clean
	rm -f purls.fst
	rm -rf target

.PHONY:  build-fst build-python clean