use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

const FILE_PATH: &str = "notes.txt";

fn main() {
    loop {
        println!("Text File CRUD Menu: ");
        println!("1. Create (overwrite)");
        println!("2. Read");
        println!("3. Update line");
        println!("4. Delete line");
        println!("5. Exit");

        match input("Choose an option: ").as_str() {
            "1" => {
                let content = multiline_input("Enter new content: ");

                fs::write(FILE_PATH, content + "\n")
                    .expect("Failed to write");

                println!("File overwritten");
            },

            "2" => {
                if let Ok(file) = fs::File::open(FILE_PATH) {
                    println!("File content:");

                    for (i, line) in BufReader::new(file).lines().enumerate() {
                        println!("{}: {}", i + 1, line.unwrap());
                    }
                } else {
                    println!("File not found");
                }
            },

            "3" => {
                let line_no = input("Line to update: ").parse::<usize>().unwrap_or(0);

                let new_text = input("New content: ");
                
                update_line(line_no, &new_text);
            },

            "4" => {
                let line_no = input("Line to delete: ").parse::<usize>().unwrap_or(0);

                delete_line(line_no);
            },

            "5" => {
                println!("Exiting...");
                break;
            },

            _ => {
                println!("Invalid option, please try again.");
            }
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

fn multiline_input(prompt: &str) -> String {
    println!("{} (type END on a new line to finish):", prompt);

    let mut content = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "END" {
            break;
        }
        content.push_str(&line);
    }
    content
}

fn update_line(line_no: usize, new_text: &str) {
    let file = OpenOptions::new().read(true).open(FILE_PATH);

    if let Ok(file) = file {
        let mut lines: Vec<String> = BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect();

        if line_no > 0 && line_no <= lines.len() {
            lines[line_no -1] = new_text.to_string();
            fs::write(FILE_PATH, lines.join("\n") + "\n")
                .expect("Failed to write updated content");

                    println!("Line {} updated", line_no);
        } else {
            println!("Invalid line number or file not found");
        }
    } 
}

fn delete_line(line_no: usize) {
    let file = OpenOptions::new().read(true).open(FILE_PATH);

    if let Ok(file) = file {
        let mut lines: Vec<String> = BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect();

        if line_no > 0 && line_no <= lines.len() {
            lines.remove(line_no - 1);
            fs::write(FILE_PATH, lines.join("\n") + "\n")
                .expect("Failed to write updated content");

            println!("Line {} deleted", line_no);
        } else {
            println!("Invalid line number or file not found");
        }
    }
}