use clap::Parser;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
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
