pub fn flowcontrol_eight() {
    let mut count = 0u32;

    loop {
        count += 1;

        if count == 3 {
            print!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("five");

            break;
        }
    }
    assert_eq!(count, 5);

    println!("Success");
}
