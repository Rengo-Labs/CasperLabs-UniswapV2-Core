prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p wcspr --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/wcspr-token.wasm 2>/dev/null | true

build-test-contract:
	cargo build --release -p test --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/wcspr-test.wasm 2>/dev/null | true

build-test-contract2:
	cargo build --release -p test2 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/wcspr-test2.wasm 2>/dev/null | true

test-only:
	cargo test -p wcspr-tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm wcspr-tests/wasm
	# cp test-contract/target/wasm32-unknown-unknown/release/*.wasm wcspr-tests/wasm

test: build-contract build-test-contract build-test-contract2 copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf wcspr-tests/wasm/*.wasm

copy-test-contract-wasm-file-to-test:
	cp test-contract/target/wasm32-unknown-unknown/release/*.wasm wcspr-tests/wasm
