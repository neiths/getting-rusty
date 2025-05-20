use std::io;
use rand::Rng;


fn main() {
    println!("Welcome to Rock-Paper_Scissors");
    println!("Instructions: Enter 'rock, 'paper', or 'scissors'. Type 'quit' to exit");

    loop {
        println!("\n rock-paper-scissors make you choice.");

        let user_choice = get_user_choice();

        if user_choice == "quit" {
            println!("thanks for playing, goodbye");
            break;
        }

        let computer_choice = get_computer_choice();

        println!("Computer chose: {}", computer_choice);

        match determine_winner(&user_choice, &computer_choice) {
            GameResult::Win => println!("You win"),
            GameResult::Lose => println!("You lost"),
            GameResult::Draw => println!("It's a draw"),
        }
    }
}

fn get_user_choice() -> String {
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "rock" | "paper" | "scissors" | "quit" => choice,
        _ => {
            println!("Invalid choice, please input rock, paper, scissors, quit");
            get_user_choice()
        }
    }
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];

    let index = rand::rng().random_range(0..choices.len());

    choices[index].to_string()
}

enum GameResult {
    Win, 
    Lose,
    Draw,
}

fn determine_winner(user_choice: &str, computer_choice: &str) -> GameResult {
    match (user_choice, computer_choice) {
        ("rock", "scissors") => GameResult::Win,
        ("paper", "rock") => GameResult::Win,
        ("scissors", "paper") => GameResult::Win,
        (a, b) if a == b => GameResult::Draw,
        _ => GameResult::Lose
    }
}