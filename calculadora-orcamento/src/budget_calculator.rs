// src/budget_calculator.rs
use crate::transaction::{Transaction, TransactionType};
use chrono::Datelike;

pub fn calculate_monthly_balance(transactions: &[Transaction], month: u32, year: i32) -> f64 {
    transactions
        .iter()
        .filter(|t| t.date.month() == month && t.date.year() == year)
        .map(|t| match t.transaction_type {
            TransactionType::Income => t.amount,
            TransactionType::Expense => -t.amount,
        })
        .sum()
}

pub fn calculate_three_month_average(
    transactions: &[Transaction],
    current_month: u32,
    current_year: i32,
) -> f64 {
    let mut total = 0.0;
    let mut count = 0;

    for i in 0..3 {
        let month = if current_month > i {
            current_month - i
        } else {
            12 + current_month - i
        };
        let year = if current_month > i {
            current_year
        } else {
            current_year - 1
        };

        let monthly_balance = calculate_monthly_balance(transactions, month, year);
        total += monthly_balance;
        count += 1;
    }

    total / count as f64
}

pub fn forecast_next_month_balance(
    transactions: &[Transaction],
    current_month: u32,
    current_year: i32,
) -> f64 {
    let average = calculate_three_month_average(transactions, current_month, current_year);
    average
}
