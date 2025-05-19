use std::{thread, time::Duration};
use std::io::{self, Write};

fn main() {
    println!("Basic Timer Tool!");
    println!("Enter the timer duration (format: hours, minutes, seconds");

    let duration = match get_timer_input() {
        Some(value) => value,
        None => {
            println!("Invalid input. please enter numbers only (e.g 0 1 30 for 1 minute 30 seconds).");
            return;
        }
    };

    println!("Timer set for: {} hours, {} minutes, {} seconds", duration.0, duration.1, duration.2);

    start_timer(duration.0, duration.1, duration.2);

    println!("Time's up");
}

fn get_timer_input() -> Option<(u64, u64, u64)> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        return None;
    }

    let hours = parts[0].parse::<u64>().ok()?;
    let minutes = parts[1].parse::<u64>().ok()?;
    let seconds = parts[2].parse::<u64>().ok()?;

    Some((hours, minutes, seconds))
}

fn start_timer(hours: u64, minutes: u64, seconds: u64) {
    let total_seconds = hours * 3600 + minutes * 60 + seconds;

    for i in (1..=total_seconds).rev() {
        let hrs = i/3600;
        let mins = (i%3600)/60;
        let secs = i%60;

        print!("\r Time remaining: {:02}:{:02}:{:02}", hrs, mins, secs);

        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    println!();
} 