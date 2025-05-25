use std::fmt;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {

    // create empty linked list
    fn new() -> Self {
        LinkedList { head: None }
    }

    // add a new node in front of linked list
    fn push_front(&mut self, val: i32) {

        // create a new node, and then its next -> first element in linked list
        let new_node = Box::new(Node {
            value: val,
            next: self.head.take(),
        });

        // turn the node into head
        self.head = Some(new_node);
    }

    fn push_back(&mut self, val: i32) {

        // create a new node
        let new_node = Box::new(Node { value: val, next: None });
        
        // and then loop all elements
        match &mut self.head {

            // if head is None, then set the new node as head
            // otherwise, find the last node and set its next to the new node
            None => self.head = Some(new_node),
            Some(_) => {
                let mut current = self.head.as_mut().unwrap();

                // is_some() method is called on an Option type in Rust. 
                // It returns true if the Option is Some(value) 
                // (i.e., it contains a value), and false if it is None.
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(new_node);
            }
        }
    }

    fn delete(&mut self, val: i32) -> bool {
        // Handle case where head needs to be deleted
        if let Some(node) = &self.head {
            if node.value == val {
                self.head = self.head.take().unwrap().next;
                return true;
            }
        }

        // Handle other cases
        let mut current = &mut self.head;
        while let Some(node) = current {
            if let Some(next_node) = &node.next {
                if next_node.value == val {
                    node.next = node.next.take().unwrap().next;
                    return true;
                }
            }
            current = &mut node.next;
        }
        false
    }

    fn contains(&self, val: i32) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.value == val {
                return true;
            }
            current = &node.next;
        }
        false
    }

    fn print(&self) {
        print!("List: ");
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}

// Optional display trait for prettier output
impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        while let Some(node) = current {
            write!(f, "{} -> ", node.value)?;
            current = &node.next;
        }
        write!(f, "None")
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(10);
    list.push_back(20);
    list.push_back(30);
    list.print(); // List: 10 -> 20 -> 30 -> None

    println!("Contains 20? {}", list.contains(20));
    println!("Deleting 20...");
    list.delete(20);
    list.print(); // List: 10 -> 30 -> None

    let mut list2 = LinkedList::new();

    list2.push_front(1);
    list2.push_front(2);
    list2.push_front(3);
    list2.push_back(4);
    list2.print(); // List: 3 -> 2 -> 1 -> 4 -> None


    list2.push_back(5);
    list2.print(); // List: 3 -> 2 -> 1 -> 4 -> 5 -> None

    list2.delete(4);
    list2.print(); // List: 3 -> 2 -> 1 -> 5 -> None
}