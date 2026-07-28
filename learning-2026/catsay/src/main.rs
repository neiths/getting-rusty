use clap::Parser;
use std::io::{self, Read};

#[derive(Parser)]
#[command(author, version, about = "A cat that speaks!", long_about = None)]
struct Args {
    /// The message for the cat to say
    message: Option<String>,
}

fn print_catsay(message: &str) {
    let len = message.len();
    let border = "-".repeat(len + 2);

    println!(" _{}_ ", border);
    println!("< {} >", message);
    println!(" -{}- ", border);
    println!("        \\");
    println!("         \\");
    println!("          /\\_/\\");
    println!("         ( o.o )");
    println!("          > ^ <");
}

fn main() {
    let args = Args::parse();

    let message = match args.message {
        Some(msg) => msg,
        None => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();

            if handle.read_to_string(&mut buffer).is_ok() && !buffer.trim().is_empty() {
                buffer.trim().to_string()
            } else {
                "Meow!".to_string()
            }
        }
    };

    print_catsay(&message);
}
