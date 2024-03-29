use common::*;

pub enum WcsprEvents {
    Deposit { purse: URef, amount: U512 },
    Withdraw { purse: URef, amount: U512 },
}

impl WcsprEvents {
    pub fn type_name(&self) -> String {
        match self {
            WcsprEvents::Deposit {
                purse: _,
                amount: _,
            } => "deposit",
            WcsprEvents::Withdraw {
                purse: _,
                amount: _,
            } => "withdraw",
        }
        .to_string()
    }
}
