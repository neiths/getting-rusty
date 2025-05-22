use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <file_path> [--lines] [--search <keyword>]");
        return;
    }

    let file_path = &args[1];

    let show_lines = args.contains(&"--lines".to_string());

    // println!("Reading file: {}", file_path);

    // println!("Show lines: {}", show_lines);

    let keyword = if let Some(pos) = args.iter().position(|x| x == "--search") {
        args.get(pos + 1)
    } else {
        None
    };

    // println!("Keyword: {:?}", keyword);


    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    // println!("File opened successfully.");

    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        
        let line = line.unwrap();

        let matched = keyword.map_or(true, |k| line.contains(k));

        if matched {
            if show_lines {
                println!("{}: {}", i+1, line);
            } else {
                println!("{}", line);
            }
        }

    }
}
