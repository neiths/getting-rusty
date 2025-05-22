use std::io::{self, Write};

fn main() {
    println!("String Manipulation Tool");

    loop {
        println!("\nChoose an operation:");
        println!("1. Reverse");
        println!("2. Uppercase");
        println!("3. Lowercase");
        println!("4. Trim");
        println!("5. Find Substring");
        println!("6. Replace Text");
        println!("7. Exit");

        let choice = prompt("Enter you choice: ");

        match choice.trim() {
            "1" => {
                let s = prompt("Enter a string: ");
                println!("Reversed: {}", s.chars().rev().collect::<String>());
            },
            "2" => {
                let s = prompt("Enter a string: ");
                println!("Uppercase: {}", s.to_uppercase());
            },
            "3" => {
                let s = prompt("Enter a string: ");
                println!("Lowercase: {}", s.to_lowercase());
            },
            "4" => {
                let s = prompt("Enter a tring: ");
                println!("Trimmed: {}", s.trim());
            }, 
            "5" => {
                let s = prompt("Enter a main string: ");
                let sub = prompt("Enter a sub string: ");

                if s.contains(&sub) {
                    println!("Substring '{}' found!", sub);
                } else {
                    println!("Substring not found");
                }
            },
            "6" => {
                let s = prompt("Enter a main string: ");

                let old = prompt("Text to replace: ");

                let new = prompt("Replacement text: ");

                println!("Result: {}", s.replace(&old, &new));
            },
            "7" => {
                println!("Goodbye");
                break;
            },
            _ => {
                println!("Invalid choice");
            }
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}