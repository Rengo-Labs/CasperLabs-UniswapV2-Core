use common::*;

pub enum PAIREvent {
    Mint {
        sender: Key,
        amount0: U256,
        amount1: U256,
        pair: Key,
    },
    Burn {
        sender: Key,
        amount0: U256,
        amount1: U256,
        to: Key,
        pair: Key,
    },
    Swap {
        sender: Key,
        amount0_in: U256,
        amount1_in: U256,
        amount0_out: U256,
        amount1_out: U256,
        to: Key,
        from: Key,
        pair: Key,
    },
    Sync {
        reserve0: U128,
        reserve1: U128,
        pair: Key,
    },
}

impl PAIREvent {
    pub fn type_name(&self) -> String {
        match self {
            PAIREvent::Mint {
                sender: _,
                amount0: _,
                amount1: _,
                pair: _,
            } => "mint",
            PAIREvent::Burn {
                sender: _,
                amount0: _,
                amount1: _,
                to: _,
                pair: _,
            } => "burn",
            PAIREvent::Swap {
                sender: _,
                amount0_in: _,
                amount1_in: _,
                amount0_out: _,
                amount1_out: _,
                to: _,
                from: _,
                pair: _,
            } => "swap",
            PAIREvent::Sync {
                reserve0: _,
                reserve1: _,
                pair: _,
            } => "sync",
        }
        .to_string()
    }
}
