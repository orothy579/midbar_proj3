use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's start!");

    let rand_num = rand::rng().random_range(1..100);
    println!("random number : {rand_num}");

    loop {
        println!("Please input the number: ");

        let mut g_num = String::new();

        io::stdin()
            .read_line(&mut g_num)
            .expect("failed to read line.");

        //shadowing..
        let g_num: u32 = match g_num.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("Your guess number : {g_num}");

        match g_num.cmp(&rand_num) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
        }
    }
}
