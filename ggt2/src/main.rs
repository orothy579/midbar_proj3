use rand::Rng;
use std::io;
fn main() {
    let rand_num = rand::rng().random_range(1..=100);
    println!("rand_num = {rand_num}");

    let mut guess;
    io::stdin().read_line(guess).expect("error");
}
