use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rn = rand::rng().random_range(1..=100);

    println!("random number is {rn}.");

    loop {
        println!("Please input your number");
        let mut guess_num = String::new();

        io::stdin()
            .read_line(&mut guess_num)
            .expect("Failed to read lines");

        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess_num.cmp(&rn) {
            Ordering::Equal => {
                println!("win");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
        }
    }
}
