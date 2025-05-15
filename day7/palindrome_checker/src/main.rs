use std::io;

fn main() {
    println!("Palidrom checker");
    println!("Enter a string to check if it's a palindrome");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("faield to read input");

    let cleaned_input = clean_string(&input);

    if cleaned_input.is_empty() {
        println!("please enter a valid non-empty string");
        return;
    }

    if is_palindrome(&cleaned_input) {
        println!("'{}' is a palindrome", input.trim());
    } else {
        println!("'{}' is not a palindrome", input.trim());
    }
}

fn clean_string(input: &str) -> String {
    input
        .chars() // iterate over each character
        .filter(|c| c.is_alphanumeric()) // keep only letters and numbers
        .map(|c| c.to_lowercase().to_string()) // convert to lowercase
        .collect::<String>() // collect into a new String
}

fn is_palindrome(s: &str) -> bool {
    // iterate each char, and reserve them, and then push into new String
    s == s.chars().rev().collect::<String>() 
} 