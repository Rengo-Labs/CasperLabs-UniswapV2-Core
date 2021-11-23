uniswap_core_directory = ../

erc20_contract = ${uniswap_core_directory}erc20/
factory_contract = ${uniswap_core_directory}factory/
flash_swapper_contract = ${uniswap_core_directory}flash\ swapper/
pair_contract = ${uniswap_core_directory}pair/
wcspr_contract = ${uniswap_core_directory}wcspr/

wasm_src_path = target/wasm32-unknown-unknown/release/


prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p pair --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/pair-token.wasm 2>/dev/null | true

build-test-contract:
	cargo build --release -p test --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/pair-token.wasm 2>/dev/null | true


test-only:
	cargo test -p pair-tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm pair-tests/wasm
	cp ${flash_swapper_contract}${wasm_src_path}*.wasm pair-tests/wasm
	cp ${factory_contract}${wasm_src_path}*.wasm pair-tests/wasm
	cp ${wcspr_contract}${wasm_src_path}*.wasm pair-tests/wasm
	cp ${erc20_contract}${wasm_src_path}*.wasm pair-tests/wasm

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
