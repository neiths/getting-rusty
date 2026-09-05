use std::{collections::HashMap, str::Matches};

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("red")),
        SpreadSheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadSheetCell::Int(i) => println!("{}", i),
        _ => println!("Not a integer!"),
    };

    let blue = String::from("Blue");
    let green = String::from("Green");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(green, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(0);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
