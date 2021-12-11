prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p test --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/wcspr-test.wasm 2>/dev/null | true
