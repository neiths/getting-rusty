pub fn flowcontrol_ten() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}