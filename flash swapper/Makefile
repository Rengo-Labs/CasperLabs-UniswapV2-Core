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
	cargo build --release -p flash_swapper --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/flash_swapper-token.wasm 2>/dev/null | true

build-test-contract:
	cargo build --release -p test --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/test-contract.wasm 2>/dev/null | true

test-only:
	cargo test -p flash_swapper-tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm flash_swapper-tests/wasm
	cp ${pair_contract}${wasm_src_path}*.wasm flash_swapper-tests/wasm
	cp ${factory_contract}${wasm_src_path}*.wasm flash_swapper-tests/wasm
	cp ${wcspr_contract}${wasm_src_path}*.wasm flash_swapper-tests/wasm
	cp ${erc20_contract}${wasm_src_path}*.wasm flash_swapper-tests/wasm

test: build-contract build-test-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf flash_swapper-tests/wasm/*.wasm
