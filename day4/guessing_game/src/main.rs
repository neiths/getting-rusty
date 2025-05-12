use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("welcome to the guessing game");
    println!("i am thinking of a number between 1 to 10. Can you guess it?");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter valid number");
                continue;
            }
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again"),
            Ordering::Greater =>  println!("Too big, try again"),
            Ordering::Equal => {
                println!("congratulations! you guessed the number");
                break;
            }
        };
    }  
}
