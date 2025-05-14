// BMI = weight(kgs)/height^2(m2)
// BMI < 18.5 UW
// BMI 18.5 - 24.9
// BMI 25-29.9
// BMI >= 30 Obesity

use::std::io;

fn main() {
    println!("BMI Calculator");
    println!("Please enter your weight in kilograms (kg):");

    let weight = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("Invalid input for weight. Please enter a valid number.");
            return;
        }
    };

    println!("Please enter you height in meters (m):");

    let height = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("Invalid input for height. Please enter a valid number");
            return;
        }
    };

    if height == 0.0 {
        println!("heigh can not be zero");
        return;
    }

    let bmi = calculate_bmi(weight, height);
    println!("your BMI is: {:.2}", bmi);

    let category = classify_bmi(bmi);
    println!("BMI category: {}", category);
}

fn get_input_as_f64() -> Option<f64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    match input.trim().parse::<f64>() {
        Ok(value) => Some(value),
        Err(_) => None
    }
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn classify_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "Underweight"
    } else if bmi < 25.0 {
        "Normal weight"
    } else if bmi < 30.0 {
        "Overweight"
    } else {
        "Obesity"
    }
}
