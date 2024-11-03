// src/transaction.rs
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub description: String,
    pub category: String,
    pub amount: f64,
    pub date: NaiveDate,
    pub transaction_type: TransactionType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Income,
    Expense,
}
