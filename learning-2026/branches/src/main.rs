fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let num = 6;

    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is divisible by 4, 3, or 2");
    }

    // using if in a let statements
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    repeating_loop(10);

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    for el in a {
        println!("the value is: {el}");
    }
}

fn repeating_loop(max_counter: i32) {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == max_counter {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
