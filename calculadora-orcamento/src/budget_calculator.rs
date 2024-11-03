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
    // TODO  Implementar cálculo da média dos últimos três meses
    return 0.0;
}

pub fn forecast_next_month_balance(average: f64) -> f64 {
    // TODO - Implementar previsão com base na média
    return 0.0;
}
