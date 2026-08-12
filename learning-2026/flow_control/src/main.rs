mod flowcontrol_five;
mod flowcontrol_four;
mod flowcontrol_one;
mod flowcontrol_three;
mod flowcontrol_two;

use flowcontrol_five::flowcontrol_five;
use flowcontrol_four::flowcontrol_four;
use flowcontrol_one::flowcontrol_one;
use flowcontrol_three::flowcontrol_three;
use flowcontrol_two::flowcontrol_two;

fn main() {
    flowcontrol_one();
    flowcontrol_two();
    flowcontrol_three();
    flowcontrol_four();
    flowcontrol_five();
}
