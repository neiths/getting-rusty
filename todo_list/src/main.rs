use std::io;

struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

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

    fn add(&mut self, title: String) {
        self.todos.push(Todo {
            id: self.next_id,
            title,
            completed: false,
        });
        self.next_id += 1;
    }

    fn list(&self) {
        if self.todos.is_empty() {
            println!("No tasks yet!");
            return;
        }

        println!("My todo List:");
        for task in &self.todos {
            println!(" {}: {} [{}]", 
                task.id, 
                task.title, 
                if task.completed {"x"} else {" "}
            );
        }
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
}

fn show_menu() {
    println!("Todo List Menu:");
    println!("1. Add a task");
    println!("2. List tasks");
    println!("3. Complete a task");
    println!("4. Delete a task");
    println!("5. Exit");
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut list = TodoList::new();

    loop {
        show_menu();

        let choice = read_line();

        match choice.as_str() {
            "1" => {
                println!("Enter task title:");
                let title = read_line();
                list.add(title);
            }
            "2" => {
                list.list();
            }
            "3" => {
                println!("Enter task ID to complete:");
                let id: u32 = read_line().parse().unwrap_or(0);
                list.complete(id);
            }
            "4" => {
                println!("Enter task ID to delete:");
                let id: u32 = read_line().parse().unwrap_or(0);
                list.delete(id);
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}