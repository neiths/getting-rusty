use std::fs;
use std::io::{self, Write};
use std::path::Path;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";

#[derive(Clone, Debug, PartialEq, Eq)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList{
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let data = self
            .todos
            .iter()
            .map(|task| format!("{}|{}|{}\n", task.id, task.completed, task.title))
            .collect::<Vec<_>>()
            .join("");

        let metadata = format!("next_id:{}\n", self.next_id);
        fs::write(path, format!("{}{}", metadata, data))
    }

    fn load<P: AsRef<Path>>(path: P) -> TodoList {
        let contents = fs::read_to_string(path).unwrap_or_default();
        if contents.trim().is_empty() {
            return TodoList::new();
        }

        let mut list = TodoList::new();
        let mut lines = contents.lines();

        if let Some(first_line) = lines.next() {
            if let Some(rest) = first_line.strip_prefix("next_id:") {
                if let Ok(next_id) = rest.parse::<u32>() {
                    list.next_id = next_id;
                }
            }
        }

        for line in lines {
            let parts: Vec<&str> = line.splitn(3, '|').collect();
            if parts.len() != 3 {
                continue;
            }

            let id = match parts[0].parse::<u32>() {
                Ok(value) => value,
                Err(_) => continue,
            };
            let completed = match parts[1].parse::<bool>() {
                Ok(value) => value,
                Err(_) => continue,
            };
            let title = parts[2].to_string();

            list.todos.push(Todo {
                id,
                title,
                completed,
            });
        }

        if list.todos.iter().map(|task| task.id).max().is_some() {
            let max_id = list.todos.iter().map(|task| task.id).max().unwrap();
            list.next_id = max_id + 1;
        }

        list
    }

    fn add(&mut self, title: String) -> bool {
        let cleaned = title.trim().to_string();
        if cleaned.is_empty() {
            return false;
        }

        self.todos.push(Todo {
            id: self.next_id,
            title: cleaned,
            completed: false,
        });
        self.next_id += 1;
        true
    }

    fn list(&self) {
        if self.todos.is_empty() {
            println!("\n{}No tasks yet. Add one to get started.{}\n", YELLOW, RESET);
            return;
        }

        println!("\n{}=== Your Tasks ==={}", BOLD, RESET);
        println!("{}ID{}  {}STATUS{}  TASK", BLUE, RESET, GREEN, RESET);
        println!("{}--{}  {}------{}  -------------------------", BLUE, RESET, GREEN, RESET);

        for task in &self.todos {
            let status_text = if task.completed { "Done" } else { "Todo" };
            let status_color = if task.completed { GREEN } else { YELLOW };
            let marker = if task.completed { "✓" } else { "•" };

            println!(
                "{}{:>2}{}  {}{:>4}{}  {} {}",
                BLUE,
                task.id,
                RESET,
                status_color,
                status_text,
                RESET,
                marker,
                task.title
            );
        }
        println!();
    }

    fn complete(&mut self, id: u32) {
        for task in &mut self.todos {
            if task.id == id {
                task.completed = true;
                println!("Completed: {}", task.title);
                return;
            }
        }
        println!("Task {} not found", id);
    }

    fn toggle(&mut self, id: u32) {
        let mut found = false;
        for task in &mut self.todos {
            if task.id == id {
                task.completed = !task.completed;
                println!("{}: {}", if task.completed { "Completed" } else { "Reopened" }, task.title);
                found = true;
                break;
            }
        }

        if !found {
            println!("Task {} not found", id);
        }
    }

    fn delete(&mut self, id: u32) {
        let len_before = self.todos.len();

        self.todos.retain(
            |t| t.id != id
        );

        if self.todos.len() < len_before {
            println!("Deleted task {}", id);
        } else {
            println!("Task {} not found", id);
        }
    }

    fn total(&self) -> usize {
        self.todos.len()
    }

    fn completed_count(&self) -> usize {
        self.todos.iter().filter(|task| task.completed).count()
    }

    fn pending_count(&self) -> usize {
        self.todos.iter().filter(|task| !task.completed).count()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::TodoList;

    #[test]
    fn add_rejects_blank_titles() {
        let mut list = TodoList::new();

        assert!(!list.add("   ".to_string()));
        assert_eq!(list.total(), 0);
    }

    #[test]
    fn counts_completed_and_pending_tasks() {
        let mut list = TodoList::new();
        list.add("Write Rust notes".to_string());
        list.add("Review exercise".to_string());
        list.complete(1);

        assert_eq!(list.total(), 2);
        assert_eq!(list.completed_count(), 1);
        assert_eq!(list.pending_count(), 1);
    }

    #[test]
    fn save_and_load_round_trip() {
        let path = PathBuf::from(std::env::temp_dir()).join("todo_list_persistence_test.txt");
        let _ = fs::remove_file(&path);

        let mut list = TodoList::new();
        list.add("Write Rust notes".to_string());
        list.add("Review exercise".to_string());
        list.complete(1);

        list.save(&path).unwrap();
        let loaded = TodoList::load(&path);

        assert_eq!(loaded.total(), 2);
        assert_eq!(loaded.completed_count(), 1);
        assert_eq!(loaded.pending_count(), 1);
        assert_eq!(loaded.todos[0].title, "Write Rust notes");

        let _ = fs::remove_file(&path);
    }
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();
}

fn show_menu() {
    println!("{}╔══════════════════════════════╗{}", BLUE, RESET);
    println!("{}║{} {}Todo Manager{} {}║{}", BLUE, RESET, BOLD, RESET, BLUE, RESET);
    println!("{}╚══════════════════════════════╝{}", BLUE, RESET);
    println!("{}  1){} Add task", CYAN, RESET);
    println!("{}  2){} View tasks", CYAN, RESET);
    println!("{}  3){} Mark done", CYAN, RESET);
    println!("{}  4){} Toggle task", CYAN, RESET);
    println!("{}  5){} Delete task", CYAN, RESET);
    println!("{}  6){} Summary", CYAN, RESET);
    println!("{}  7){} Exit", CYAN, RESET);
    print!("\n{}Select an option: {}", YELLOW, RESET);
    io::stdout().flush().unwrap();
}

fn show_summary(list: &TodoList) {
    println!("\n{}=== Summary ==={}", BOLD, RESET);
    println!("{}Total tasks:   {}{}", CYAN, list.total(), RESET);
    println!("{}Completed:     {}{}", GREEN, list.completed_count(), RESET);
    println!("{}Pending:       {}{}", YELLOW, list.pending_count(), RESET);
    println!("{}Progress:      {:.0}%{}", BLUE, if list.total() == 0 { 0.0 } else { (list.completed_count() as f64 / list.total() as f64) * 100.0 }, RESET);
    println!();
    io::stdout().flush().unwrap();
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let file_path = "todos.txt";
    let mut list = TodoList::load(file_path);

    loop {
        list.list();
        show_menu();

        let choice = read_line();
        println!();

        match choice.as_str() {
            "1" => {
                println!("{}Task title: {}", CYAN, RESET);
                let title = read_line();
                if !list.add(title) {
                    println!("\n{}Task cannot be empty. Please enter a meaningful title.{}\n", RED, RESET);
                } else {
                    println!("\n{}Task added successfully.{}\n", GREEN, RESET);
                    if let Err(err) = list.save(file_path) {
                        println!("{}Failed to save tasks: {}{}", RED, err, RESET);
                    }
                }
            }
            "2" => {
                list.list();
            }
            "3" => {
                println!("{}Task ID to mark as done: {}", CYAN, RESET);
                let id: u32 = read_line().parse().unwrap_or(0);
                list.complete(id);
                if let Err(err) = list.save(file_path) {
                    println!("{}Failed to save tasks: {}{}", RED, err, RESET);
                }
            }
            "4" => {
                println!("{}Task ID to toggle: {}", CYAN, RESET);
                let id: u32 = read_line().parse().unwrap_or(0);
                list.toggle(id);
                if let Err(err) = list.save(file_path) {
                    println!("{}Failed to save tasks: {}{}", RED, err, RESET);
                }
            }
            "5" => {
                println!("{}Task ID to delete: {}", CYAN, RESET);
                let id: u32 = read_line().parse().unwrap_or(0);
                list.delete(id);
                if let Err(err) = list.save(file_path) {
                    println!("{}Failed to save tasks: {}{}", RED, err, RESET);
                }
            }
            "6" => {
                show_summary(&list);
            }
            "7" => {
                clear_screen();
                println!("{}Goodbye! Your tasks were saved.{}", GREEN, RESET);
                io::stdout().flush().unwrap();
                break;
            }
            _ => {
                println!("\n{}Invalid choice. Please choose a number from the menu.{}\n", RED, RESET);
            }
        }

        println!("{}Press Enter to continue...{}", YELLOW, RESET);
        let _ = read_line();
        io::stdout().flush().unwrap();
    }
}