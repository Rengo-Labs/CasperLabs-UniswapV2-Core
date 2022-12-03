wasm_src_path = target/wasm32-unknown-unknown/release/

wasm_dest_erc20 = erc20/erc20-tests/wasm/
wasm_dest_wcspr = wcspr/wcspr-tests/wasm/
wasm_dest_factory = factory/factory-tests/wasm/
wasm_dest_flashswapper = flashswapper/flashswapper-tests/wasm/
wasm_dest_pair = pair/pair-tests/wasm/

prepare:
	rustup target add wasm32-unknown-unknown

build-contract-erc20:
	cargo build --release -p erc20 --target wasm32-unknown-unknown
build-contract-wcspr:
	cargo build --release -p wcspr -p session-code-wcspr --target wasm32-unknown-unknown
build-contract-factory:
	cargo build --release -p factory --target wasm32-unknown-unknown
build-contract-flashswapper:
	cargo build --release -p flashswapper --target wasm32-unknown-unknown
build-contract-pair:
	cargo build --release -p pair --target wasm32-unknown-unknown

build-all:
	make build-contract-erc20
	make build-contract-wcspr
	make build-contract-factory
	make build-contract-flashswapper
	make build-contract-pair

copy-wasm-file-erc20:
	cp ${wasm_src_path}erc20-token.wasm ${wasm_dest_erc20}
copy-wasm-file-wcspr:
	cp ${wasm_src_path}wcspr-token.wasm ${wasm_dest_wcspr}
	cp ${wasm_src_path}session-code-wcspr.wasm ${wasm_dest_wcspr}
copy-wasm-file-factory:
	cp ${wasm_src_path}factory.wasm ${wasm_dest_factory}
copy-wasm-file-flashswapper:
	cp ${wasm_src_path}flashswapper-token.wasm ${wasm_dest_flashswapper}
	cp ${wasm_src_path}factory.wasm ${wasm_dest_flashswapper}
	cp ${wasm_src_path}wcspr-token.wasm ${wasm_dest_flashswapper}
copy-wasm-file-pair:
	cp ${wasm_src_path}*.wasm ${wasm_dest_pair}

copy-wasm-file-all:
	make copy-wasm-file-erc20
	make copy-wasm-file-wcspr
	make copy-wasm-file-factory
	make copy-wasm-file-flashswapper
	make copy-wasm-file-pair

test-erc20:
	cargo test -p erc20-tests
test-wcspr:
	cargo test -p wcspr-tests
test-factory:
	cargo test -p factory-tests
test-flashswapper:
	cargo test -p flashswapper-tests
test-pair:
	cargo test -p pair-tests

test-all:
	make test-erc20
	make test-wcspr
	make test-factory
	make test-flashswapper
	make test-pair

all:
	make build-all
	make copy-wasm-file-all
	make test-all

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf erc20-tests/wasm/*.wasm
	rm -rf wcspr-tests/wasm/*.wasm
	rm -rf factory-tests/wasm/*.wasm
	rm -rf flashswapper-tests/wasm/*.wasm
	rm -rf pair-tests/wasm/*.wasm

git-clean:
	git rm -rf --cached .
	git add .