use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    //collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <file_path>");
        return;
    }

    let file_path = &args[1];
    println!("Reading file: {}", file_path);

    //read the file contents
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error openning file: {}", err);
            return;
        }
    };

    let mut contents = String::new();

    if let Err(err) = file.read_to_string(&mut contents) {
        println!("Error reading file: {}", err);
        return;
    }

    //count words
    let word_count = count_word(&contents);
    println!("Word count: {}", word_count);

    let char_count = count_character(&contents);
    println!("character count: {}", char_count);

    println!("show each character in the text: \n");
    print_each_char(&contents);

    println!("finding word in contents");

    let a: &str = find_word("thien", &contents);
    println!("case 1: {}", a);

    let b: &str = find_word("individual", &contents);
    println!("case 2: {}", b);
}

fn count_word(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_character(text: &str) -> usize {
    let chars: Vec<char> = text.chars().collect();
    chars.len()
}

fn print_each_char(text: &str) {
    for c in text.chars() {
        print!("{} ", c);
    }
}

fn find_word<'a>(keyword: &str, text: &'a str) -> &'a str {
    for word in text.split_whitespace() {
        if word == keyword {
            println!("found!");
            return word;
        }
    }

    println!("not found!");
    return "";
}