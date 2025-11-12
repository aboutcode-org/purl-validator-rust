build-fst:
	cargo run --bin fst_builder

clean:
	cargo clean
	rm -f purls.fst
	rm -rf target

test:
	cargo test -- --nocapture

valid:
	cargo fmt --all
	cargo clippy --all-targets --all-features

check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

doc:
	cargo doc --open

.PHONY:  build-fst clean test valid check doc