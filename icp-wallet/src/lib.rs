use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use std::cell::RefCell;
use std::collections::HashMap;

type Address = String;

#[derive(CandidType, Deserialize, Clone)]
struct Wallet {
    balances: HashMap<Address, u64>,
}

impl Wallet {
    fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    fn get_balance(&self, address: &Address) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    fn send_tokens(&mut self, from: Address, to: Address, amount: u64) -> Result<(), String> {
        let sender_balance = self.get_balance(&from);
        if sender_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        self.balances.insert(from.clone(), sender_balance - amount);
        let receiver_balance = self.get_balance(&to);
        self.balances.insert(to, receiver_balance + amount);
        Ok(())
    }

    fn receive_tokens(&mut self, address: Address, amount: u64) {
        let balance = self.get_balance(&address);
        self.balances.insert(address, balance + amount);
    }
}

// Storage
thread_local! {
    static WALLET: RefCell<Wallet> = RefCell::new(Wallet::new());
}

// Smart contract functions
#[ic_cdk::update]
fn send_tokens(from: Address, to: Address, amount: u64) -> String {
    WALLET.with(|wallet| {
        wallet.borrow_mut().send_tokens(from, to, amount)
    }).unwrap_or_else(|e| e)
}

#[ic_cdk::update]
fn receive_tokens(address: Address, amount: u64) {
    WALLET.with(|wallet| wallet.borrow_mut().receive_tokens(address, amount));
}

#[ic_cdk::query]
fn get_balance(address: Address) -> u64 {
    WALLET.with(|wallet| wallet.borrow().get_balance(&address))
}
