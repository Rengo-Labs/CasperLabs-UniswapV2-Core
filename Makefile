uniswap_core_directory = .

erc20_contract = ${uniswap_core_directory}/erc20/
factory_contract = ${uniswap_core_directory}/factory/
flash_swapper_contract = ${uniswap_core_directory}/flash\ swapper/
pair_contract = ${uniswap_core_directory}/pair/
wcspr_contract = ${uniswap_core_directory}/wcspr/

wasm_src_path = target/wasm32-unknown-unknown/release/

build-contract:
	# Build erc20
	cd ${erc20_contract} && make build-contract

	# Build factory
	cd ${factory_contract} && make build-contract

	# Build flash swapper
	cd ${flash_swapper_contract} && make build-contract

	# Build pair
	cd ${pair_contract} && make build-contract

	# Build wcspr
	cd ${wcspr_contract} && make build-contract

	# copy wasm files
	make copy-wasm-file
clean:
	# clean erc20
	cd ${erc20_contract} && make clean

	# clean factory
	cd ${factory_contract} && make clean

	# clean flash swapper
	cd ${flash_swapper_contract} && make clean

	# clean pair
	cd ${pair_contract} && make clean

	# clean wcspr
	cd ${wcspr_contract} && make clean


# copy wasm to required directory with new names
copy-wasm-file:
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${erc20_contract}erc20-tests/wasm/
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${pair_contract}pair-tests/wasm/
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${factory_contract}factory-tests/wasm/
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${flash_swapper_contract}flash_swapper-tests/wasm/
	cp ${erc20_contract}${wasm_src_path}*.wasm ${pair_contract}pair-tests/wasm/
	cp ${erc20_contract}${wasm_src_path}*.wasm ${factory_contract}factory-tests/wasm/
	cp ${erc20_contract}${wasm_src_path}*.wasm ${flash_swapper_contract}flash_swapper-tests/wasm/
	cp ${pair_contract}${wasm_src_path}*.wasm ${flash_swapper_contract}flash_swapper-tests/wasm/
	cp ${pair_contract}${wasm_src_path}*.wasm ${factory_contract}factory-tests/wasm/
	cp ${factory_contract}${wasm_src_path}*.wasm ${pair_contract}pair-tests/wasm/
	cp ${factory_contract}${wasm_src_path}*.wasm ${flash_swapper_contract}flash_swapper-tests/wasm/

# run all tests sequentially
test:

	# Test WCSPR
	cd ${wcspr_contract} && make test

	# Test ERC20
	cd ${erc20_contract} && make test

	# Test Flash Swapper
	cd ${flash_swapper_contract} && make test

	# Test Pair
	cd ${pair_contract} && make test

	# Test Factory
	cd ${factory_contract} && make test
