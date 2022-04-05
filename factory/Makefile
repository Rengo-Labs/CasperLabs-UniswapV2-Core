# uniswap_core_directory = ../

# erc20_contract = ${uniswap_core_directory}erc20/
# factory_contract = ${uniswap_core_directory}factory/
# flash_swapper_contract = ${uniswap_core_directory}flash-swapper/
# pair_contract = ${uniswap_core_directory}pair/
# wcspr_contract = ${uniswap_core_directory}wcspr/

# wasm_src_path = target/wasm32-unknown-unknown/release/


prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p factory --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/factory.wasm 2>/dev/null | true

test-only:
	cargo test -p factory-tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm factory-tests/wasm
	# cp ${flash_swapper_contract}${wasm_src_path}*.wasm factory-tests/wasm
	# cp ${pair_contract}${wasm_src_path}*.wasm factory-tests/wasm
	# cp ${wcspr_contract}${wasm_src_path}*.wasm factory-tests/wasm
	# cp ${erc20_contract}${wasm_src_path}*.wasm factory-tests/wasm


test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf factory-tests/wasm/*.wasm
