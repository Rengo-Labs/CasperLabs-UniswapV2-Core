# CasperLabs PAIR

Implementation of the PAIR Contract for the Casper platform.

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

### PAIR

The `pair` crate contains the implementation of the PAIR Contract and ERC20 standard.

#### PAIR as library
It can be used as a library to create pairs of two erc20 tokens and it can be used to build custom tokens. The code structure allows for easy entry points extensions and overrides.


##### Entry Point override example
The following code shows how to override the `transfer` method to alwasy mint
one additional token for a sender. 

```rust
#[derive(Default)]
struct Pair(ContractStorage);

impl ContractContext for Pair {
    fn storage(&self) -> &ContractStorage {
        &self.0
    }
}

impl PAIR for Pair {}

impl Pair {
    fn constructor(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256, nonce:U256, domain_separator: String, permit_type_hash: String, contract_hash: ContractHash) {
        PAIR::init(self, name, symbol, decimals, domain_separator, permit_type_hash, Key::from(contract_hash));
        PAIR::mint(self, self.get_caller(), initial_supply);
        PAIR::set_nonce(self, self.get_caller(), nonce);
    }
}
```

#### PAIR Vanilla Contract
The library comes with a vanilla implementation of the PAIR contract and ERC20 contract that is
ready to use. It is implemented in `pair/bin/pair_token.rs` and after 
compilation the `pair-token.wasm` file is produced.

### PAIR Tests
The `pair-tests` crate implements multiple integration test scenarios that
check the compatibility with the ERC20 standard.

Tests provide the `ERC20Instance` struct that can be reused in larger smart
contract projects with multiple ERC20 tokens and other smart contracts
to interact with the instance of an ERC20 token.

Tests are implemented in `pair-tests/src/pair_tests.rs`.

### Utils

The repository contains 2 utility crates:

* `utils/test-env`
* `utils/contract-utils`

The utility code after review and adoption should be moved to a separate repo
and eventually be added to `casper-contract` and `casper-engine-test-support`.

#### Test Env Crate
`utils/test-env` is a small library written on top of 
`casper-engine-test-support`. It provides two structs:

* `TestEnv` wraps `TestContext` and provides user accounts with initial
  CSPR balances. It is implemented using `Arc<Mutex<...>>` so it can
  be copied, especial between `TestContract` instances.
* `TestContract` wraps an instance of `TestEnv` and simplifies calling
  contracts and reading named keys and dictionaries.

##### Test Example
```rust
struct Token(TestContract);

impl Token {
    pub fn new(env: &TestEnv, sender: Sender) -> Token {
        Token(TestContract::new(env, "token.wasm", "token_name", sender, runtime_args! {
            "initial_supply" => U256::from(1000)
        }))
    }

    pub fn transfer(&self, sender: Sender, recipient: AccountHash, amount: u64) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }

    pub fn balance_of(&self, account: AccountHash) -> u64 {
        self.0
            .query_dictionary("balances", account.to_string())
            .unwrap_or_default()
    }
}

#[test]
fn test_multiple_tokens() {
    // Prepare the env and users.
    let env = TestEnv::new();
    let user1 = env.next_user();
    let user2 = env.next_user();
    
    // Deploy multiple instances of the same contract
    // agains a single virtual machine.
    let token1 = Token::new(&env, Sender(user1));
    let token2 = Token::new(&env, Sender(user2));

    // Transfer tokens.
    let amount = 10;
    token1.transfer(Sender(user1), user2, amount);
    assert_eq!(token1.balance_of(user1), amount);
}
```

#### Contract Utils Crate
`utils/contract-utils` contains common building blocks for writing smart
contracts:
* `contract_context.rs` provides the `ContractContext` trait that has 
  `get_caller` and `self_addr` methods.
* `data.rs` provides helper methods to work with dictionaries and named
  keys.
* `admin_control.rs` provides the `AdminControl` trait to support admin
  list functionality.

