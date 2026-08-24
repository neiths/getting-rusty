#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(u8, u8, u8, u8),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn describe(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({x}, {y})"),
            Message::Write(text) => println!("Write message: {text}"),
            Message::ChangeColor(r, g, b) => {
                println!("Change color to RGB({r}, {g}, {b})")
            }
        }
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn new(kind: IpAddrKind, address: &str) -> Self {
        Self {
            kind,
            address: address.to_string(),
        }
    }

    fn print_details(&self) {
        match &self.kind {
            IpAddrKind::V4(a, b, c, d) => {
                println!("IPv4: {}.{}.{}.{} -> {}", a, b, c, d, self.address)
            }
            IpAddrKind::V6(a, b, c, d) => {
                println!("IPv6: {}.{}.{}.{} -> {}", a, b, c, d, self.address)
            }
        }
    }
}

fn main() {
    let localhost = IpAddr::new(IpAddrKind::V4(127, 0, 0, 1), "localhost");
    let home = IpAddr::new(IpAddrKind::V6(0, 0, 0, 0), "home");

    localhost.print_details();
    home.print_details();

    let mut messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];

    messages.push(Message::Write(String::from("another message")));

    messages.push(Message::ChangeColor(0, 255, 0));

    for message in messages {
        message.describe();
        println!("------------------------------");
    }
}
