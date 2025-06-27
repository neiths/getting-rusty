use std::io;

fn main() {
    println!("Simple Calculator");
    println!("avaiable operations: +, -, *, /");
    println!("Entern your expression (eg. 5 + 3): ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let token: Vec<&str> = input.trim().split_whitespace().collect();

    if token.len() != 3 {
        println!("Invalid input. Please follow the format: number operator number");
        return;
    }

    let num1: f64 = match token[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid first parameter.");
            return;
        }
    };

    let num2: f64 = match token[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid third parameter");
            return;
        }
    };

    let op = token[1];

    let result = match op {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => mul(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("Operator is not existed in the program!!!");
            return;
        }
    };

    println!("Result: {:.2}", result);

}

fn add(n1: f64, n2: f64) -> f64 {
    n1 + n2
}

fn subtract(n1: f64, n2: f64) -> f64 {
    n1 - n2
}

fn mul(n1: f64, n2: f64) -> f64 {
    n1 * n2
}

fn divide(n1: f64, n2: f64) -> f64 {
    if n2 == 0.0{
        println!("Divison by zero is not allowed");
        std::process::exit(1);
    }

    n1 / n2
}