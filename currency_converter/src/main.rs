mod api;
mod converter;
mod models;

use converter::CurrencyConverter;
use std::io;

fn main() {
    println!("Currency Converter");

    let mut from_currency = String::new();
    let mut to_currency = String::new();
    let mut amount = String::new();

    println!("Enter the base currency (e.g., USD):");
    io::stdin().read_line(&mut from_currency).unwrap();
    let from_currency = from_currency.trim().to_uppercase();

    println!("Enter the target currency (e.g., EUR):");
    io::stdin().read_line(&mut to_currency).unwrap();
    let to_currency = to_currency.trim().to_uppercase();

    println!("Enter the amount to convert:");
    io::stdin().read_line(&mut amount).unwrap();
    let amount: f64 = amount.trim().parse().unwrap_or(0.0);

    if amount <= 0.0 {
        println!("Invalid amount entered.");
        return;
    }

    let converter = CurrencyConverter::new();
    match converter.convert(&from_currency, &to_currency, amount) {
        Ok(result) => println!(
            "{} {} = {:.2} {}",
            amount, from_currency, result, to_currency
        ),
        Err(err) => println!("Error: {}", err),
    }
}
