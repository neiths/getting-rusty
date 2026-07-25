use std::fmt 

fn main() {
    println!("{} days", 29);

    // Positional arguments can be used. Specifying an integer inside '{}'
    // determines which additional arguments wil be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", 
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over"
    );

    println!("Base 10: {}", 69420);
    println!("Base 2 (binary) {:b}", 69420);
    println!("Base 8 {:o}", 69420);
    println!("Base 16 {:x}", 69420);

    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:0<5}", number=1);

    println!("{number:0>width$}", number=1, width=5);

    #[allow(dead_code)] // disble `dead_code` which warn against unused module
    struct Structure(i32);

    fmt::Display.
    println!("");("This struct `{}` won't print...", Structure(3));

}
