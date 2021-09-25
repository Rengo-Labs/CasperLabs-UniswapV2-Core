prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p pair --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/pair-token.wasm 2>/dev/null | true

build-test-contract:
	cargo build --release -p test --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/test-token.wasm 2>/dev/null | true

test-only:
	cargo test -p pair-tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm pair-tests/wasm

test: build-contract build-test-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf pair-tests/wasm/*.wasm
