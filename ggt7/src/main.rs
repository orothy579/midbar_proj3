// random number 맞추는 게임
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rand_num = rand::rng().random_range(1..=100);
    println!("random number is {rand_num}");

    loop {
        println!("Please input your number:");
        let mut guess_num = String::new();

        io::stdin()
            .read_line(&mut guess_num)
            .expect("failed to read lines");

        let guess_num: i32 = match guess_num.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess_num.cmp(&rand_num) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }

            Ordering::Less => {
                println!("Too small");
            }

            Ordering::Greater => {
                println!("Too big");
            }
        }
    }
}
