// src/wasm_interface.rs
use crate::budget_calculator::calculate_monthly_balance;
use crate::transaction::Transaction;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
    static TRANSACTIONS: RefCell<Vec<Transaction>> = RefCell::new(Vec::new());
}

#[wasm_bindgen]
pub fn add_transaction(transaction_json: &str) {
    let transaction: Transaction = serde_json::from_str(transaction_json).unwrap();
    TRANSACTIONS.with(|t| t.borrow_mut().push(transaction));
}

#[wasm_bindgen]
pub fn get_monthly_balance(month: u32, year: i32) -> f64 {
    TRANSACTIONS.with(|t| calculate_monthly_balance(&t.borrow(), month, year))
}
