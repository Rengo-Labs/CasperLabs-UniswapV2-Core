# CasperLabs FLWASH SWAPPER

Implementation of the FLASH SWAPPER standard for the Casper platform.

## Usage
### Install
Make sure `wasm32-unknown-unknown` is installed.
```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

### Build Smart Contract
```
make build-contract
```

### Test
Test logic and smart contract.
```
make test
```

## Repository overview

### FLASH SWAPPER

The `flash_swapper` crate contains the implementation of the FLASHSWAPPER standard.

#### FLASHSWAPPER as library
It can be used as a library to build custom tokens. The code structure allows
for easy entry points extensions and overrides.


#### FLASHSWAPPER Vanilla Contract
The library comes with a vanilla implementation of the ERC20 contract that is
ready to use. It is implemented in `flash_swapper/bin/flash_swapper.rs` and after 
compilation the `flash-swapper.wasm` file is produced.

### FLASH SWAPPER Tests
The `flash_swapper-tests` crate implements multiple integration test scenarios that
check the compatibility with the FLASHS WAPPER standard.

Tests provide the `FLASHSWAPPERInstance` struct that can be reused in larger smart
contract projects with multiple FLASH SWAPPERs and other smart contracts
to interact with the instance of an FLASH SWAPPER.

Tests are implemented in `flash_swapper-tests/src/flash_swapper_tests.rs`.

### Utils

The repository contains 2 utility crates:

* `utils/test-env`
* `utils/contract-utils`

The utility code after review and adoption should be moved to a separate repo
and eventually be added to `casper-contract` and `casper-engine-test-support`.

