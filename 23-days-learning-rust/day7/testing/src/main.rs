fn main() {
    // chars() return an iterator you can not print it directly. 
    println!("{:?}", "racecar".chars().collect::<Vec<char>>());

    println!("{}", "racecar".chars().collect::<String>());
}