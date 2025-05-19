use std::{thread, time::Duration};
use std::io::{self, Write};
use colored::*;

fn main() {
    clear_terminal();
    println!("{}", "🕒 Basic Timer Tool".bold().cyan());
    println!("{}", "Enter time as: hours minutes seconds (e.g. 0 1 30)".dimmed());

    let duration = match get_timer_input() {
        Some(value) => value,
        None => {
            println!("{}", "❌ Invalid input. Example: 0 1 30".red().bold());
            return;
        }
    };

    let (h, m, s) = duration;
    println!(
        "{} {}h {}m {}s",
        "✅ Timer set for:".green(),
        h.to_string().yellow(),
        m.to_string().yellow(),
        s.to_string().yellow()
    );

    println!("{}", "🔔 Countdown begins...".bold().blue());
    start_timer(h, m, s);

    println!("\n{}", "⏰ Time's up!".bold().green());
}

fn get_timer_input() -> Option<(u64, u64, u64)> {
    let mut input = String::new();

    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).ok()?;

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
        let hrs = i / 3600;
        let mins = (i % 3600) / 60;
        let secs = i % 60;

        let spinner = match i % 4 {
            0 => "⠋",
            1 => "⠙",
            2 => "⠸",
            _ => "⠴",
        };

        print!(
            "\r{} {} {:02}:{:02}:{:02} remaining...",
            spinner.blue(),
            "⏳".dimmed(),
            hrs,
            mins,
            secs
        );
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(1));
    }
    println!();
}

fn clear_terminal() {
    let _ = std::process::Command::new("clear").status();
}
