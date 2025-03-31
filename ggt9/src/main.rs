// guessing game
// 1. random number 생성
// 2. 사용자로부터 number 입력받기
// 3. 비교
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rn = rand::rng().random_range(1..=100);
    println!("random number is {rn}");

    loop {
        let mut gn = String::new();

        io::stdin()
            .read_line(&mut gn)
            .expect("Failed to read line.");

        let gn: i32 = match gn.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match gn.cmp(&rn) {
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
        }
    }
}
