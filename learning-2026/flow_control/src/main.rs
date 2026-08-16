mod flowcontrol_eight;
mod flowcontrol_five;
mod flowcontrol_four;
mod flowcontrol_one;
mod flowcontrol_seven;
mod flowcontrol_six;
mod flowcontrol_three;
mod flowcontrol_two;
mod flowcontrol_nine;
mod flowcontrol_ten;

use flowcontrol_eight::flowcontrol_eight;
use flowcontrol_five::flowcontrol_five;
use flowcontrol_four::flowcontrol_four;
use flowcontrol_one::flowcontrol_one;
use flowcontrol_seven::flowcontrol_seven;
use flowcontrol_six::flowcontrol_six;
use flowcontrol_three::flowcontrol_three;
use flowcontrol_two::flowcontrol_two;
use flowcontrol_nine::flowcontrol_nine;
use flowcontrol_ten::flowcontrol_ten;

fn main() {
    println!("1. ===============================");
    flowcontrol_one();
    println!("2. ===============================");
    flowcontrol_two();
    println!("3. ===============================");
    flowcontrol_three();
    println!("4. ===============================");
    flowcontrol_four();
    println!("5. ===============================");
    flowcontrol_five();
    println!("6. ===============================");
    flowcontrol_six();
    println!("7. ===============================");
    flowcontrol_seven();
    println!("8. ===============================");
    flowcontrol_eight();
    println!("9. ===============================");
    flowcontrol_nine();
    println!("10. ===============================");
    flowcontrol_ten();

}
