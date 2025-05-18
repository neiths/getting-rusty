use std::fs::{self, File};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use colored::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: String,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        
        println!("{}", "============================".cyan());
        println!("{}", "      TO-DO LIST MENU       ".bold().yellow());
        println!("{}", "============================".cyan());
        println!("1. {}", "Add task".green());
        println!("2. {}", "View tasks".green());
        println!("3. {}", "Mark task as complete".green());
        println!("4. {}", "Delete task".green());
        println!("5. {}", "Exit".red());
        println!("{}", "----------------------------".cyan());

        let choice = get_input("Enter your choice: ");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_complete(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("{}", "Tasks saved. Goodbye! 👋".bold().blue());
                break;
            }
            _ => println!("{}", "Invalid choice. Please try again.".red()),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks)
        .expect("Failed to serialize tasks");

    let mut file = File::create("tasks.json")
        .expect("Failed to save tasks");

    file.write_all(json.as_bytes())
        .expect("failed to write tasks to file");
}

fn add_task(tasks: &mut Vec<Task>) {
    let description = get_input("Enter task description: ");

    let id = Uuid::new_v4().to_string();

    tasks.push(
        Task {
            id, 
            description: description.trim().to_string(),
            completed: false,
        }
    );

    println!("{}", "✅ Task added successfully!".green().bold());
}

fn view_tasks(tasks:  &Vec<Task>) {
    println!("\n{}", "====== TASK LIST ======".bold().blue());

    if tasks.is_empty() {
        println!("No tasks found");
    } else {
        for task in tasks {
            let status = if task.completed {
                "✅ Completed".green()
            } else {
                "❌ Not done".red()
            };

            println!(
                "{}. [{}] {}",
                task.id.to_string().yellow().bold(),
                status,
                task.description.bold()
            );
        }
    }

    println!();
}

fn mark_task_complete(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to mark as complete: ");

    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("{}", "✅ Task marked as complete!".green().bold());
        } else {
            println!("{}", "⚠️ Task not found.".red());
        }
    } else {
        println!("{}", "🚫 Invalid task ID.".red());
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to delete: ");

    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(index) = tasks.iter().position(|t| t.id == id) {
            tasks.remove(index);
            println!("{}", "🗑️ Task deleted successfully.".green().bold());
        } else {
            println!("{}", "⚠️ Task not found.".red());
        }
    } else {
        println!("{}", "🚫 Invalid task ID.".red());
    }
}