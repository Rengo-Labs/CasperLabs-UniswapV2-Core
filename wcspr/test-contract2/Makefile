prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p test2 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/wcspr-test2.wasm 2>/dev/null | true
