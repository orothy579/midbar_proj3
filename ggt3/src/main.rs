use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let rand_num: u32 = rand::rng().random_range(1..=100);
    println!("random number : {}", rand_num);

    loop {
        println!("Please input the number : ");

        let mut guess_num = String::new();
        io::stdin().read_line(&mut guess_num).expect("error");

        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_num.cmp(&rand_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
