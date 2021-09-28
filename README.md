# CasperLabs Contracts

Implementation of ERC20 Token, Pair ,Factory and Flash Swapper, WCSPR Contract for the CasperLabs platform.

## Steps
There are 5 contracts in this folder 
1) ERC20 Token Contract
2) Pair Contract
3) Factory Contract
4) FLASH SWAPPER Contract
5) WCSPR Contract

To run the Contracts make sure you are in the folder of your required contract.
See the makefile and README file in that folder for further Details.

## How to get public_key and signature for permit function
There are 5 total steps 
1) Deploy erc20 or pair contract whose permit function you want to call.
2) Query the permit type hash, domain separator and nonces on their keys.
3) Clone the get_public_key_and_signature branch in uniswap_coreV2 repo and Provide the required data,  data should be  exactly the same as in or will be in erc20 or pair which you just have deployed.
4) Build the code using the command : cargo build.
5) Run the code using the command : cargo run.

Now you have the public key and signature.
