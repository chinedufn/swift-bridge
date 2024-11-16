#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type BankAccount;

        #[swift_bridge(get(amount))]
        fn amount(&self) -> u32;

        fn make_bank_account() -> BankAccount;
    }
}

pub(crate) struct BankAccount {
    pub amount: u32,
}

fn make_bank_account() -> BankAccount {
    BankAccount { amount: 500 }
}
