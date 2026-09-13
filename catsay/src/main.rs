use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String, // [1]

    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
}

fn main() {
    let options = Options::parse(); // [2]
    let message = options.message;

    let eye = if options.dead { "x" } else { "o" }; // 1

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    match &options.catfile {
        Some(path) => {
            let cat_template =
                std::fs::read_to_string(path).expect(&format!("could not read file {:?}", path));
            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", cat_picture);
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye = eye.red().bold()); // [2]
            println!("    =( I )=");
        }
    }
}
