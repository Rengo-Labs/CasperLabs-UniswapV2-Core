# Scytalelabs-UniswapV2Core
Implementation of `ERC20 Token`, `Pair` ,`Factory`, `Flash Swapper`, and `WCSPR` Contract for the CasperLabs platform.

## Steps
There are 5 contracts in this folder 
1) ERC20 Token Contract
2) Pair Contract
3) Factory Contract
4) FLASH SWAPPER Contract
5) WCSPR Contract

To run the Contracts make sure you are in the folder of your required contract.

## Interacting with the contract
You need to have `casper-client` and `jq` installed on your system to run the examples. The instructions have been tested on Ubuntu 20.04.2 LTS.

### Install the prerequisites

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

```bash
# Update package repositories
sudo apt update

# Install the command-line JSON processor
sudo apt install jq -y

# Add Casper repository
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
sudo apt-key add casper-repo-pubkey.asc
sudo apt update

# Install the Casper client software
sudo apt install casper-client -y
```

### Known contract hashes

All contracts have already being deployed. Inorder to interact with the specific contract you need to call it by its hash. The table below contains the contract hash (without the `hash-` prefix) for all the contracts on public Casper networks:

Network| Contract Name | Account info contract hash | Contract owner
---|---|---|---
Testnet| ERC20 | `hash-621dd1f5d73c70b43c9b85b6bf549326b65d3cd7fb2cd28067b378998ad3bf8d` | Casper 
Association
Testnet| WCSPR | `hash-083756dee38a7e3a8a7190a17623cfbc8bc107511de206f03c3dbd1af5463a45` | Casper 
Association
Testnet| FLASHSWAPPER | ` hash-fbfeda8b97f056f526f20c2fc2b486d9bdbfb3e46b9a164527e57c0c86e68612` | Casper 
Association
Testnet| PAIR | `hash-8e6fbaae9f5ff3bb3cca7cb15723b2a47917d074922575187cb136e8d4b169a7` | Casper 
Association
Testnet| FACTORY | `hash-5028190b8a5b6addbf3d51ee2c6ae5b913f09223d65eff9bcf5985f74ae976ec` | Casper 
Association


### Deploying ERC20 contract manually

If you need to deploy the `ERC20 contract` manually you need to pass the some parameters. Following is the command to deploy the `ERC20 contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:u256='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods

Following are the ERC20's entry point methods.

- #### transfer
Lets ` self.get_caller() ` send pool tokens to a recipient hash.

Following is the table of parameters.

Parameter Name | Type
---|---
recipient | Key 
amount | U256


This method **returns** nothing.

- #### transfer_from
Sends pool tokens from one hash to another.
<br>User needs to call approve method before calling the ` tranfer_from `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key 
recipient | Key 
amount | U256


This method **returns** nothing.


- #### permit
Sets the allowance for a spender where approval is granted via a signature.

Following is the table of parameters.

Parameter Name | Type
---|---
public | String 
signature | String
owner | Key
spender | Key
value | U256
deadline | u64


This method **returns** nothing.


- #### approve
Lets ` self.get_caller() ` set their allowance for a spender.
<br>user needs to call this `approve` method before calling the `transfer_from` method.

Following is the table of parameters.

Parameter Name | Type
---|---
spender | Key 
amount | U256

This method **returns** nothing.

- #### balance_of
This method will return the balance of owner in `ERC20 Contract`.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key 


This method **returns** U256.


- #### nonce
Returns the current `nonce` for an address for use in ` permit `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key 


This method **returns** U256.


- #### allowance
Returns the amount of liquidity tokens owned by an hash that a spender is allowed to transfer via ` transfer_from `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key 
spender | Key 


This method **returns** U256.


- #### total_supply
Returns the total amount of pool tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** U256.


- #### mint
This method mints the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key 
amount | U256

This method **returns** nothing.


- #### burn
This method burns the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key 
amount | U256

This method **returns** nothing.
<br>**Note:** To `burn` the tokens against the hash provided by user, User needs to `mint` tokens first in `ERC20`.

- #### name
Returns the `name` of tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** String.

- #### symbol
Returns the `symbol` of tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** String.


### Deploying WCSPR contract manually

If you need to deploy the `WCSPR contract` manually you need to pass the some parameters. Following is the command to deploy the `WCSPR contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:u256='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods

Following are the WCSPR's entry point methods.

- #### transfer
Lets ` self.get_caller() ` send pool tokens to a recipient hash.

Following is the table of parameters.

Parameter Name | Type
---|---
recipient | Key 
amount | U256


This method **returns** nothing.


- #### transfer_from
Sends pool tokens from one hash to another.
<br>User needs to call `approve` method before calling the `tranfer_from`.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key 
recipient | Key 
amount | U256


This method **returns** nothing.


- #### approve
Lets `self.get_caller()` set their allowance for a spender.

<br>user needs to call this `approve` method before calling the `transfer_from` method.

Following is the table of parameters.

Parameter Name | Type
---|---
spender | Key 
amount | U256

This method **returns** nothing.

- #### balance_of
This method will return the balance of owner in `ERC20 Contract` .

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key 

This method **returns** U256.

- #### allowance
Returns the amount of liquidity tokens owned by an hash that a spender is allowed to transfer via ` transfer_from `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key 
spender | Key 

This method **returns** U256.

- #### total_supply
Returns the total amount of pool tokens for a pair.

This method **returns** U256.


- #### deposit
This method deposits the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key 
amount | U256

This method **returns** nothing.


- #### withdraw
This method withdraws the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key 
amount | U256

This method **returns** nothing.
<br>**Note:** To `withdraw` the tokens against the hash provided by user, User needs to `deposit` tokens first in `WCSPR`.

- #### name
Returns the `name` of tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** String.


- #### symbol
Returns the `symbol` of tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** String.


### Deploying PAIR contract manually

If you need to deploy the `PAIR contract` manually you need to pass the hashes of the other contracts as parameter. Following is the command to deploy the `PAIR contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:u256='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
    --session-arg="factory_hash:Key='Hash of factory Contract'" \
    --session-arg="callee_contract_hash:Key='Flash Swapper Contract Hash'" \
```

Before deploying `PAIR Contract`, you would need to deploy other contracts first and pass hashes of these contracts to the respective parameters above. We have already deployed these contracts and the tables belows displays the hashes of the contracts.

Name | Network | Account info contract hash | Contract owner
---|---|---|---
Factory | Testnet | `hash-5028190b8a5b6addbf3d51ee2c6ae5b913f09223d65eff9bcf5985f74ae976ec` | Casper Association
Flash Swapper | Testnet | ` hash-fbfeda8b97f056f526f20c2fc2b486d9bdbfb3e46b9a164527e57c0c86e68612` | Casper Association



### Manual Deployment

For manual deployments of these contracts, following are the commands.

#### Factory
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="fee_to_setter:Key='Hash of fee-to-setter Contract'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Flash Swapper
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:u256='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods

Following are the PAIR's entry point methods.

- #### transfer
Lets ` self.get_caller ` send pool tokens to a recipient hash.

Following is the table of parameters.

Parameter Name | Type
---|---
recipient | Key 
amount | U256


This method **returns** nothing.


- #### transfer_from
Sends pool tokens from one hash to another.
<br>**Note:** User needs to call `approve` method before calling the `tranfer_from` .

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key 
recipient | Key 
amount | U256


This method **returns** nothing.

- #### swap
Swaps tokens. For regular swaps, ` data.length ` must be ` 0 `.
<br> **Note:** User needs to deploy a `Factory contract` first and call a method `create_pair` which invokes the `initialize` methods of `Pair contract` that's how the `Pair contract` can access the `token0` and `token1` after this user needs to mint `token0` and `token1` by calling an `erc20_mint` method in `pair contract` so they have some balance in them. To call the `swap` method the user needs to call the `sync` method which updates the `reserve0` and `reserve1` with the amount that was minted by the `erc20_mint` method.

Following is the table of parameters.

Parameter Name | Type
---|---
amount0_out | U256 
amount1_out | U256 
to | Key
data | String


This method **returns** nothing.

- #### skim
<br>**Note:** User needs to deploy a `Factory contract` first and call a method `create_pair` which invokes the `initialize` methods of `Pair contract` that's how the `Pair contract` can access the `token0` and `token1` after this user needs to mint `token0` and `token1` by calling an `erc20_mint` method in `Pair contract` so they have some balance in them. To call the `skim` method the user needs to call the `sync` method which updates the `reserve0` and `reserve1` with the amount that was minted by the `erc20_mint` method.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key


This method **returns** nothing.


- #### skim
<br>**Note:** User needs to deploy a `Factory contract` first and call a method `create_pair` which invokes the `initialize` methods of `Pair contract` that's how the `Pair contract` can access the `token0` and `token1` after this user needs to mint `token0` and `token1` by calling an `erc20_mint` method in `Pair contract` so they have some balance in them. Then call the `sync` method which updates the `reserve0` and `reserve1` with the amount that was minted by the `erc20_mint` method.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** nothing.


- #### permit
Sets the allowance for a spender where approval is granted via a signature.

Following is the table of parameters.

Parameter Name | Type
---|---
public | String 
signature | String
owner | Key
spender | Key
value | U256
deadline | u64


This method **returns** nothing.


- #### approve
Lets ` self.get_caller() ` set their allowance for a spender.
<br>user needs to call this `approve` method before calling the `transfer_from` method.

Following is the table of parameters.

Parameter Name | Type
---|---
spender | Key 
amount | U256


This method **returns** nothing.


- #### balance_of
Returns the amount of pool tokens owned by a hash.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key 


This method **returns** U256.


- #### nonce
Returns the current `nonce` for an address for use in ` permit `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key 


This method **returns** U256.


- #### allowance
Returns the amount of liquidity tokens owned by an hash that a spender is allowed to transfer via ` transfer_from `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key 
spender | Key 


This method **returns** U256.


- #### total_supply
Returns the total amount of pool tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** U256.


- #### mint
Creates pool tokens.
<br>**Note:** User needs to deploy a `Factory contract` first and call a method `create_pair` which invokes the `initialize` methods of `Pair contract` that's how the `Pair contract` can access the `token0` and `token1` after this user needs to mint `token0` and `token1` by calling an `erc20_mint` method in `Pair contract` so they have some balance in them. Then call the `sync` method which updates the `reserve0` and `reserve1` with the amount that was minted by the `erc20_mint` method. To call the mint user needs to do all the above steps so he can proceed flawlessly.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key

This method **returns** U256.

- #### burn
Destroys pool tokens.
<br>**Note:** User needs to mint tokens before burning them. And user needs to deploy a `Factory contract` first and call a method `create_pair` which invokes the `initialize` method of `Pair contract` that's how the `Pair contract` can access the `token0` and `token1` after this user needs to mint `token0` and `token1` by calling an `erc20_mint` method in `Pair contract` so they have some balance in them. Then call the `sync` method which updates the `reserve0` and `reserve1` with the amount that was minted by the `erc20_mint` method. To call the burn user needs to do all the above steps so he can proceed flawlessly.


Following is the table of parameters.

Parameter Name | Type
---|---
to | Key 

This method **returns** Tuple(U256, U256).


- #### treasury_fee
Returns the Treasury Fee for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** U256.


- #### set_treasury_fee_percent
sets the treasury fee for a pair.
<br>**Note:** treasury_fee_percent Cannot be more than `30`. If it’s more than `30` it will set it as `30`.

Following is the table of parameters.

Parameter Name | Type
---|---
treasury_fee | U256


This method **returns** nothing.


- #### token0
Returns the hash of the pair token with the `lower sort order`.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** Key.


- #### token1
Returns the address of the pair token with the `higher sort order`.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** Key.


- #### initialize
Sets the `token0` and `token1` in pair contract.
<br>**Note:**  This method will be called by `Factory contract` only and the user needs to pass the factory hash to make sure is it a factory or not.

Following is the table of parameters.

Parameter Name | Type
---|---
token0 | Key
token1 | Key
factory_hash | Key

This method **returns** nothing.


- #### get_reserves
Returns the reserves of token0 and token1 used to price trades and distribute liquidity. Also returns the block_time_stamp `(mod 2**32)` of the last block during which an interaction occured for the pair.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** Tupe3(U128, U128, u64).


- #### erc20_mint
This method mints the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key 
amount | U256

This method **returns** nothing.


### Deploying FACTORY contract manually

If you need to deploy the `FACTORY contract` manually you need to pass the some parameters. Following is the command to deploy the `FACTORY contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="fee_to_setter:Key='Account Hash of a user'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods

Following are the FACTORY's entry point methods.

- #### create_pair
Creates a pair for `token_a` and `token_b` if one doesn't exist already.
<br>**Note:** `token_a` and `token_b` are interchangeable and The user needs to deploy the pair contract before calling the create pair method so he can pass the `Pair contract` hash as a parameter which allows the `Factory contract` to call the `initialize` methods of `Pair Contract`.
Following is the table of parameters.

Parameter Name | Type
---|---
token_a | Key 
token_b | Key 
pair_hash | Key 


This method **returns** nothing.


- #### get_pair
Returns the hash of the pair for `token0` and `token1`, if it has been created, else `“Hash-0000000000000000000000000000000000000000000000000000000000000000”`.
<br>**Note:** `token0` and `token1` are interchangeable.

Following is the table of parameters.

Parameter Name | Type
---|---
token0 | Key 
token1 | Key 


This method **returns** Key.


- #### fee_to
Returns the hash of `fee_to`.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** Key.


- #### fee_to_setter
Returns the hash of `fee_to_setter`.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** Key.

- #### all_pairs
Returns the list of all pairs created.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** list of Keys.


- #### all_pairs_length
Returns the total number of pairs created through the `factory` so far.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** U256.


- #### set_fee_to
this will set the hash of `fee_to`
<br>**Note:** Only `fee_to_setter` can set the `fee_to`

Following is the table of parameters.

Parameter Name | Type
---|---
fee_to | Key


This method **returns** nothing.


- #### set_fee_to_setter
this will set the Hash of `fee_to_setter`
<br>**Note:** Only `fee_to_setter` can set the `fee_to_setter`

Following is the table of parameters.

Parameter Name | Type
---|---
fee_to_setter | Key


This method **returns** nothing.


### Deploying Flash Swapper contract manually

If you need to deploy the `Flash swapper contract` manually you need to pass the hashes of the other contracts as parameter. Following is the command to deploy the `Flash Swapper contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="uniswap_v2_factory:Key='Hash of factory Contract'" \
    --session-arg="wcspr:Key='Hash of WCSPR Contract'" \
    --session-arg="dai:Key='Hash of DAI Contract'" \
    --session-arg="contract_name:string='contract_name'"
```


Before deploying `Flash Swapper Contract`, you would need to deploy other contracts first and pass hashes of these contracts to the respective parameters above. We have already deployed these contracts and the tables belows displays the hashes of the contracts.

Name | Network | Account info contract hash | Contract owner
---|---|---|---
Factory | Testnet | `hash-5028190b8a5b6addbf3d51ee2c6ae5b913f09223d65eff9bcf5985f74ae976ec` | Casper Association
Wcspr | Testnet | `hash-083756dee38a7e3a8a7190a17623cfbc8bc107511de206f03c3dbd1af5463a45` | Casper Association
Dai | Testnet | `hash-083756dee38a7e3a8a7190a17623cfbc8bc107511de206f03c3dbd1af5463a45` | Casper Association


### Manual Deployment

For manual deployments of these contracts, following are the commands.

#### Factory
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="fee_to_setter:Key='Hash of fee-to-setter Contract'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Wcspr
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:u256='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Dai
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:u256='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods

Following are the Flash Swapper's entry point methods.

- #### start_swap
This method will start swap.
<br>Special Instructions: The user needs to call this (start_swap) method first to call the `uniswap_v2_call` method.
`Start_swap` method will further call 3 methods
- simple_flash_loan
This method will be invoked if both tokens (token_borrow and token_pay) are the same.
- simple_flash_swap
This method will be invoked if both tokens (token_borrow and token_pay) are not the same. one of them must be equal to 
“Hash-0000000000000000000000000000000000000000000000000000000000000000”
- triangular_flash_swap
This method will be invoked if both tokens (token_borrow and token_pay) are not the same.
The above mthods will invoke the swap methods of `Pair` Contract by using the `permissioned_pair_address`. And then the `swap` method will invoke the `uniswap_v2_call` method.

Following is the table of parameters.

Parameter Name | Type
---|---
token_borrow | Key 
amount | U256
token_pay | Key
user_data | String

This method **returns** nothing.


- #### uniswap_v2_call
This method is called by `swap` method of `pair contract`. 
<br>the sender must be a `Flash Swapper Contract` hash if user data has some value.
`Uniswap_v2_call` must be called from a contract. Users cannot directly invoke this method.


Following is the table of parameters.

Parameter Name | Type
---|---
sender | Key 
amount0 | U256
amount1 | U256
data | String

This method **returns** nothing.
