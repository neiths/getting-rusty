pub fn flowcontrol_five() {
    let a: [i32; 4] = [4, 3, 2, 1];

    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}
