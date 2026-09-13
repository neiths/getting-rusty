use clap::Parser;

#[derive(Parser)]
struct Options {
    message: String, // [1]
}

fn main() {
    let options = Options::parse(); // [2]
    let message = options.message;

    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("    =( I )=");
}
