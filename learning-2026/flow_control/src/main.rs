mod flowcontrol_eight;
mod flowcontrol_five;
mod flowcontrol_four;
mod flowcontrol_one;
mod flowcontrol_seven;
mod flowcontrol_six;
mod flowcontrol_three;
mod flowcontrol_two;

use flowcontrol_eight::flowcontrol_eight;
use flowcontrol_five::flowcontrol_five;
use flowcontrol_four::flowcontrol_four;
use flowcontrol_one::flowcontrol_one;
use flowcontrol_seven::flowcontrol_seven;
use flowcontrol_six::flowcontrol_six;
use flowcontrol_three::flowcontrol_three;
use flowcontrol_two::flowcontrol_two;

fn main() {
    flowcontrol_one();
    flowcontrol_two();
    flowcontrol_three();
    flowcontrol_four();
    flowcontrol_five();
    flowcontrol_six();
    flowcontrol_seven();
    flowcontrol_eight();
}
