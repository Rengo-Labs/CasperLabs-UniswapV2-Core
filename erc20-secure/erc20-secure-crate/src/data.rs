use casper_types::{ContractHash, ContractPackageHash};
use casperlabs_contract_utils::{get_key, set_key};

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";

pub fn set_contract_hash(contract_hash: ContractHash) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_contract_hash() -> ContractHash {
    get_key(SELF_CONTRACT_HASH).unwrap_or_default()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(CONTRACT_PACKAGE_HASH).unwrap_or_default()
}
