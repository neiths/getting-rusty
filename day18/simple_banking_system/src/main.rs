use std::io::{self, Write};

#[derive(Debug)]
struct Account {
    id: usize,
    name: String,
    balance: f64,
}

fn main() {
    let mut accounts: Vec<Account> = Vec::new();

    let mut next_id = 1;

    loop {
        println!("\n🏦 Banking System:");
        println!("1. Create Account");
        println!("2. View Balance");
        println!("3. Deposit");
        println!("4. Withdraw");
        println!("5. Exit");

        match input("Enter an option: ").as_str() {
            "1" => {
                let name = input("Account holder name: ");
                let amount = input("Initial deposit: ").parse::<f64>().unwrap();

                accounts.push(Account {id: next_id, name, balance: amount});

                println!("Account created with ID: {}", next_id);

                next_id += 1;
            },

            "2" => {
                let id = input("Enter Account ID: ").parse::<usize>().unwrap();

                match accounts.iter().find(|acc| acc.id == id) {
                    Some(acc) => println!("{} | balance: {:2}", acc.name, acc.balance),
                    None => {
                        println!("Accont not found");
                    }
                }
            },

            "3" => {
                let id = input("Account ID: ").parse::<usize>().unwrap();

                let amt = input("Amount to deposit: ").parse::<f64>().unwrap();

                match accounts.iter_mut().find(|acc| acc.id == id) {
                    Some(acc) => {
                        acc.balance += amt;
                        println!("New balance: {:2}", acc.balance);
                    },
                    None => println!("Account not found")
                }
            },

            "4" => {
                let id = input("Enter Account ID: ").parse::<usize>().unwrap();

                let amt = input("Amount to withdraw: ").parse::<f64>().unwrap();

                match accounts.iter_mut().find(|acc| acc.id == id) {
                    Some(acc) => {
                        if acc.balance >= amt {
                            acc.balance -= amt;
                            println!("Withdrawal successful. new balance: {:.2}", acc.balance);
                        } else {
                            println!("Insufficient funds");
                        }
                    },
                    None => println!("Account not found")
                }
            },

            "5" => {
                println!("Goodbye");
                break;
            },

            _ => println!("Invalid choice")
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