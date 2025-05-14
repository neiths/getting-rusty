// BMI = weight(kgs)/height^2(m2)
// BMI < 18.5 UW
// BMI 18.5 - 24.9
// BMI 25-29.9
// BMI >= 30 Obesity


// some dependencies

use axum::{
    extract::Json, // For handling JSON requests and responses
    routing::post, // For defining POST routes
    Router, // For creating the application router
};
use serde::{Deserialize, Serialize}; // For JSON serialization/deserialization
use std::net::SocketAddr; // For defining the server address
use tokio::net::TcpListener; // For creating the TCP listener

// Define the structure for incoming JSON requests
#[derive(Deserialize)]
struct BmiRequest {
    weight: f64,  // Weight value (presumably in kg)
    height: f64, // Height value (presumably in meters)
}

// Define the structure for outgoing JSON responses
#[derive(Serialize)]
struct BmiResponse {
    bmi: f64, // The calculated BMI value
    category: &'static str,  // The BMI category (e.g., "Normal weight")
}

#[tokio::main] // this macro sets up the Tokio async runtime
async fn main() {

    // create a new router and define a single POST route at "/bmi"
    let app = Router::new().route("/bmi", post(calculate_bmi_handler));

    // Define the server address as localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // creat a tcp listener bound to the address
    let listener = TcpListener::bind(addr).await.unwrap();
    
    // start the server with our app
    axum::serve(listener, app).await.unwrap();
}

async fn calculate_bmi_handler(Json(payload): Json<BmiRequest>) -> Json<BmiResponse> {
    
    // check for invalid height (division by zero)
    if payload.height == 0.0 {
        return Json(BmiResponse {
            bmi: 0.0,
            category: "Invalid height",
        });
    }

    // calculate BMI and determine category
    let bmi = calculate_bmi(payload.weight, payload.height);
    let category = classify_bmi(bmi);

    // return the response as JSON
    Json(BmiResponse { bmi, category })
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height) // BMI formula: weight(kg) / height^2(m^2)
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

// fn main() {
//     println!("BMI Calculator");
//     println!("Please enter your weight in kilograms (kg):");

//     let weight = match get_input_as_f64() {
//         Some(value) => value,
//         None => {
//             println!("Invalid input for weight. Please enter a valid number.");
//             return;
//         }
//     };

//     println!("Please enter you height in meters (m):");

//     let height = match get_input_as_f64() {
//         Some(value) => value,
//         None => {
//             println!("Invalid input for height. Please enter a valid number");
//             return;
//         }
//     };

//     if height == 0.0 {
//         println!("heigh can not be zero");
//         return;
//     }

//     let bmi = calculate_bmi(weight, height);
//     println!("your BMI is: {:.2}", bmi);

//     let category = classify_bmi(bmi);
//     println!("BMI category: {}", category);
// }

// fn get_input_as_f64() -> Option<f64> {
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("failed to read input");

//     match input.trim().parse::<f64>() {
//         Ok(value) => Some(value),
//         Err(_) => None
//     }
// }

// fn calculate_bmi(weight: f64, height: f64) -> f64 {
//     weight / (height * height)
// }

// fn classify_bmi(bmi: f64) -> &'static str {
//     if bmi < 18.5 {
//         "Underweight"
//     } else if bmi < 25.0 {
//         "Normal weight"
//     } else if bmi < 30.0 {
//         "Overweight"
//     } else {
//         "Obesity"
//     }
// }
