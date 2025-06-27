use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    id: usize,
    name: String,
    balance: f64,
}

fn main() {
    let mut accounts = load_accounts();

    let mut next_id = accounts.keys().max().map_or(1, |id| id + 1);

    loop {
        println!("\n🏦 Banking System:");
        println!("1. Create Account");
        println!("2. View Balance");
        println!("3. Deposit");
        println!("4. Withdraw");
        println!("5. Save & Exit");

        match input("Enter an option: ").as_str() {
            "1" => {
                let name = input("Account holder name: ");
                let amount = parse_amount("Initial deposit");

                let account = Account {
                    id: next_id,
                    name,
                    balance: amount
                };
                
                // built-in fun of HashMap
                accounts.insert(next_id, account);

                println!("✅ Account created with ID: {}", next_id);
                next_id += 1;
            },

            "2" => {
                let id = parse_id();

                match accounts.get(&id) {
                    Some(acc) => println!("{} (ID: {}) | Balance: {:.2}", acc.name, acc.id, acc.balance),
                    None => println!("❌ Account not found"),
                }
            },

            "3" => {
                let id = parse_id();
                let amount = parse_amount("Amount to deposit");
                match accounts.get_mut(&id) {
                    Some(acc) => {
                        acc.balance += amount;
                        println!("✅ New balance: {:.2}", acc.balance);
                    }
                    None => println!("❌ Account not found"),
                }
            },

            "4" => {
                let id = parse_id();
                let amount = parse_amount("Amount to withdraw");
                match accounts.get_mut(&id) {
                    Some(acc) => {
                        if acc.balance >= amount {
                            acc.balance -= amount;
                            println!("✅ Withdrawal successful. New balance: {:.2}", acc.balance);
                        } else {
                            println!("❌ Insufficient funds");
                        }
                    }
                    None => println!("❌ Account not found"),
                }
            },

            "5" => {
                save_accounts(&accounts);
                println!("💾 Accounts saved. Goodbye!");
                break;
            },

            _ => println!("❗ Invalid choice, try again."),
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn parse_amount(prompt: &str) -> f64 {
    loop {
        let input_str = input(&format!("{}: ", prompt));
        match input_str.parse::<f64>() {
            Ok(amount) if amount >= 0.0 => return amount,
            _ => println!("❌ Please enter a valid non-negative number."),
        }
    }
}

fn parse_id() -> usize {
    loop {
        let input_str = input("Enter Account ID: ");
        match input_str.parse::<usize>() {
            Ok(id) => return id,
            _ => println!("❌ Invalid ID. Please enter a valid number."),
        }
    }
}

fn save_accounts(accounts: &HashMap<usize, Account>) {
    if let Ok(json) = serde_json::to_string_pretty(accounts) {
        fs::write("accounts.json", json).expect("Unable to save accounts");
    } else {
        println!("Failed to serialize accounts");
    }
}

fn load_accounts() -> HashMap<usize, Account> {
    if Path::new("accounts.json").exists() {
        let data = fs::read_to_string("accounts.json").unwrap_or_else(|_| "{}".into());
        serde_json::from_str(&data).unwrap_or_else(|_| HashMap::new())
    } else {
        HashMap::new()
    }
}