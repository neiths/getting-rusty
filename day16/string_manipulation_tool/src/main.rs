use std::io::{self, Write};

fn main() {
    println!("📚 Welcome to the String Manipulation Tool!");

    loop {
        println!("\n🛠️  Choose an operation:");
        println!("  1. 🔁 Reverse");
        println!("  2. 🔠 Uppercase");
        println!("  3. 🔡 Lowercase");
        println!("  4. ✂️ Trim");
        println!("  5. 🔍 Find Substring");
        println!("  6. ✏️ Replace Text");
        println!("  7. ❌ Exit");

        let choice = prompt("👉 Enter your choice: ");

        match choice.trim() {
            "1" => {
                let s = prompt("📝 Enter a string: ");
                println!("🔁 Reversed: {}", s.chars().rev().collect::<String>());
            },
            "2" => {
                let s = prompt("📝 Enter a string: ");
                println!("🔠 Uppercase: {}", s.to_uppercase());
            },
            "3" => {
                let s = prompt("📝 Enter a string: ");
                println!("🔡 Lowercase: {}", s.to_lowercase());
            },
            "4" => {
                let s = prompt("📝 Enter a string: ");
                println!("✂️ Trimmed: '{}'", s.trim());
            }, 
            "5" => {
                let s = prompt("📝 Enter the main string: ");
                let sub = prompt("🔍 Enter the substring to find: ");
                if s.contains(&sub) {
                    println!("✅ Substring '{}' found!", sub);
                } else {
                    println!("❌ Substring '{}' not found.", sub);
                }
            },
            "6" => {
                let s = prompt("📝 Enter the main string: ");
                let old = prompt("🔁 Text to replace: ");
                let new = prompt("🆕 Replacement text: ");
                println!("✏️ Result: {}", s.replace(&old, &new));
            },
            "7" => {
                println!("👋 Goodbye!");
                break;
            },
            _ => {
                println!("❗ Invalid choice. Please try again.");
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
