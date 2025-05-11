use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");
    println!("Please select an option (1 or 2):");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice.please enter 1 or 2");
            return;
        }
    };

    if choice == 1 {
        celsius_to_fahrenheit();
    } else if choice == 2 {
        fahrenehit_to_celsius();
    } else {
        println!("Invalid choice, please select 1 or 2");
    }

}

fn celsius_to_fahrenheit() {
    println!("Enter temperature in Celsius:");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Failed to input");

    let temp: f64 = match temp.trim().parse() {
        Ok(temp) => temp, 
        Err(_) => {
            println!("Invalid input. Please enter valid temperature");
            return;
        }
    };

    // F = (C * 9/5) + 32
    let fahrenheit = ( temp  * 9.0 / 5.0) + 32.0;

    println!("{:.2} C is {:.2} F", temp, fahrenheit);
}

fn fahrenehit_to_celsius() {
    println!("Enter temperature in Fahrenheit:");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(temp) => temp,
        Err(_) => {
            println!("Invalid input. Please enter valid temperature");
            return;
        }
    };

    // C = (F - 32) * 5/9
    let celsius = (temp - 32.0) * 5.0 / 9.0;
    println!("{:.2} F is {:.2} C", temp, celsius);
}