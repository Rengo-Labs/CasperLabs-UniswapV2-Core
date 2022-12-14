use common::{
    functions::{account_zero_address, zero_address},
    keys::*,
    unwrap_or_revert::UnwrapOrRevert,
    *,
};

pub enum FACTORYEvent {
    PairCreated {
        token0: Key,
        token1: Key,
        pair: Key,
        all_pairs_length: U256,
    },
    PairRemoved {
        token0: Key,
        token1: Key,
        pair: Key,
        all_pairs_length: U256,
    },
}
impl FACTORYEvent {
    pub fn type_name(&self) -> String {
        match self {
            FACTORYEvent::PairCreated {
                token0: _,
                token1: _,
                pair: _,
                all_pairs_length: _,
            } => "pair_created",
            FACTORYEvent::PairRemoved {
                token0: _,
                token1: _,
                pair: _,
                all_pairs_length: _,
            } => "pair_removed",
        }
        .to_string()
    }
}

pub struct Whitelists {
    dict: Dict,
}

impl Whitelists {
    pub fn instance() -> Whitelists {
        Whitelists {
            dict: Dict::instance(WHITELISTS_DICT),
        }
    }

    pub fn init() {
        Dict::init(WHITELISTS_DICT)
    }

    pub fn get(&self, owner: &Key) -> (Key, Key) {
        self.dict
            .get_by_key(owner)
            .unwrap_or((account_zero_address(), zero_address()))
    }

    pub fn set(&self, owner: &Key, value: Key, pair: Key) {
        self.dict.set_by_key(owner, (value, pair));
    }
}
pub struct Pairs {
    dict: Dict,
}

impl Pairs {
    pub fn instance() -> Pairs {
        Pairs {
            dict: Dict::instance(PAIRS_DICT),
        }
    }

    pub fn init() {
        Dict::init(PAIRS_DICT)
    }

    pub fn get(&self, token0: &Key, token1: &Key) -> Key {
        self.dict
            .get_by_keys((token0, token1))
            .unwrap_or_else(zero_address)
    }

    pub fn set(&self, token0: &Key, token1: &Key, value: Key) {
        self.dict.set_by_keys((token0, token1), value);
    }
}

pub fn set_fee_to(fee_to: Key) {
    set_key(FEE_TO, fee_to);
}

pub fn get_fee_to() -> Key {
    get_key(FEE_TO).unwrap_or_else(account_zero_address)
}

pub fn set_fee_to_setter(fee_to_setter: Key) {
    set_key(FEE_TO_SETTER, fee_to_setter);
}

pub fn get_fee_to_setter() -> Key {
    get_key(FEE_TO_SETTER).unwrap_or_else(account_zero_address)
}

pub fn set_all_pairs(all_pairs: Vec<Key>) {
    set_key(ALL_PAIRS, all_pairs);
}

pub fn get_all_pairs() -> Vec<Key> {
    get_key(ALL_PAIRS).unwrap_or_revert()
}

pub fn set_owner(owner: Key) {
    set_key(OWNER, owner);
}

pub fn get_owner() -> Key {
    get_key(OWNER).unwrap_or_else(account_zero_address)
}
