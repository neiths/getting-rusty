use std::io::{self, Write};

#[derive(Debug)]
struct Contact {
    id: usize,
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    let mut next_id = 1;

    loop {
        println!("\n📇 Contact Manager:");
        println!("1. Add Contact");
        println!("2. View Contacts");
        println!("3. Search Contact");
        println!("4. Delete Contact");
        println!("5. Exit");

        let choice = input("Enter your choice: ");

        match choice.trim() {
            "1" => {
                let name = input("Name: ");
                let phone = input("Phone: ");
                let email = input("Email: ");
                contacts.push(
                    Contact {
                        id: next_id,
                        name, 
                        phone,
                        email
                    }
                );

                println!("Contact added with ID {}", next_id);

                next_id += 1;
            },

            "2" => {
                if contacts.is_empty() {
                    println!("No contacts");
                } else {
                    for contact in &contacts {
                        println!("[{}] {} | {} | {}", contact.id, contact.name, contact.phone, contact.email);
                    }
                }
            },

            "3" => {
                let query = input("Search by name or email: ");

                let results: Vec<&Contact> = contacts.iter()
                    .filter(
                        |c| 
                            c.name.contains(&query) ||
                            c.email.contains(&query)
                    ).collect();
                
                if results.is_empty() {
                    println!("❌ No match found.");
                } else {
                    for c in results {
                        println!("[{}] {} | {} | {}", c.id, c.name, c.phone, c.email);
                    }
                }
            },

            "4" => {
                let id = input("Enter ID to delete: ").parse::<usize>().unwrap();

                let len_before = contacts.len();

                contacts.retain(|c| c.id != id);

                if contacts.len() < len_before {
                    println!("Contact deleted");
                } else {
                    println!("ID not found");
                }
            },

            "5" => {
                println!("Exiting...");
                break;
            },

            _ => println!("invalid option")
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

