// src/budget_calculator.rs
use crate::transaction::{Transaction, TransactionType};
use chrono::Datelike;

pub fn calculate_monthly_balance(transactions: &[Transaction], month: u32, year: i32) -> f64 {
    transactions
        .iter()
        .filter(|t| t.date.month() == month && t.date.year() == year)
        .fold(0.0, |acc, t| match t.transaction_type {
            TransactionType::Expense => acc - t.amount,
            TransactionType::Income => acc + t.amount,
        })
}
