// guessing game
// 무작위 숫자를 맞추는 게임
// 무작위 숫자 보다 작으면 too small, 크면 too bing 같으면 정답 후 프로그램 종료

use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let rand_num = rand::rng().random_range(1..=100);

    loop {
        let mut guess_num = String::new();

        io::stdin()
            .read_line(&mut guess_num)
            .expect("Failed to read line!");

        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess_num.cmp(&rand_num) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => {
                println!("Too big");
            }
            Ordering::Less => {
                println!("Too small");
            }
        }
    }
}
