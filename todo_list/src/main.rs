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


fn main() {
    let mut list = TodoList::new();

    list.add(String::from("Learn Rust"));
    list.add(String::from("Build a todo app"));
    list.add(String::from("Profit!"));
    list.list();

    list.complete(1);
    list.list();

    list.delete(2);
    list.complete(3);
    list.list();
}