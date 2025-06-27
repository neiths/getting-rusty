use std::io;


fn main() {
    println!("Fibonacci sequence generator");
    println!("Enter the number of terms you want to generate: ");

    let num_terms = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("Invalid input. please enter a positive integer");
            return;
        }
    };


    if num_terms == 0 {
        println!("number of terms must be greater than zero");
        return;
    }

    let sequence = generate_fibonacci(num_terms);
    println!("Fibonacci sequence ({} term): {:?}", num_terms, sequence);
}


fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input.");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn generate_fibonacci(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();

    if n >= 1 {
        sequence.push(0);
    } 

    if n >= 2 {
        sequence.push(1);
    }

    for i in 2..n {
        let next = sequence[i as usize - 1] + sequence[i as usize - 2];
        sequence.push(next);
    }

    sequence
}