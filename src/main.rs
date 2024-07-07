use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use std::collections::HashMap;

type Balance = u64;

#[derive(CandidType, Deserialize, Clone)]
struct Token {
    balances: HashMap<String, Balance>,
}

#[init]
fn init() -> Token {
    Token {
        balances: HashMap::new(),
    }
}

#[update]
fn send(to: String, amount: Balance) -> bool {
    let from = ic_cdk::caller();
    let from_balance = get_balance(from.to_string());

    if from_balance < amount {
        return false;
    }

    let to_balance = get_balance(to.clone());
    set_balance(from.to_string(), from_balance - amount);
    set_balance(to, to_balance + amount);
    true
}

#[query]
fn get_balance(account: String) -> Balance {
    ic_cdk::storage::get::<Token>()
        .balances
        .get(&account)
        .cloned()
        .unwrap_or(0)
}

fn set_balance(account: String, balance: Balance) {
    ic_cdk::storage::get_mut::<Token>()
        .balances
        .insert(account, balance);
}
