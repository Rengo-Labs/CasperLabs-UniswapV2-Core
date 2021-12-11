# CasperLabs FACTORY

Implementation of the FACTORY standard for the Casper platform.

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

### FACTORY

The `factory` crate contains the implementation of the FACTORY standard.

#### factory as library
It can be used as a library to creat pairs by deploying the PAIR Contract. The code structure allows for easy entry points extensions and overrides.

#### FACTORY Vanilla Contract
The library comes with a vanilla implementation of the FACTORY contract that is
ready to use. It is implemented in `factory/bin/factory.rs` and after 
compilation the `factory.wasm` file is produced.

### FACTORY Tests
The `factory-tests` crate implements multiple integration test scenarios that
check the compatibility with the FACTORY standard.

Tests provide the `FACTORYInstance` struct that can be reused in larger smart
contract projects with multiple FACTORY and other smart contracts
to interact with the instance of an FACTORY.

Tests are implemented in `factory-tests/src/factory_tests.rs`.

### Utils

The repository contains 2 utility crates:

* `utils/test-env`
* `utils/contract-utils`

The utility code after review and adoption should be moved to a separate repo
and eventually be added to `casper-contract` and `casper-engine-test-support`.
