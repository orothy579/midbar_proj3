use rand::Rng;
use std::cmp::Ordering;
use std::io;

// 1. random number 생성
// 2. user input 받기 --> input 은 언제나 string 인가?
// 3. random number 와 user input 비교
//  - 동일하면 정답 아니면 반복

fn main() {
    println!("Let's start");
    let num = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess number:");
        let mut g_num = String::new();
        io::stdin()
            .read_line(&mut g_num)
            .expect("falied to read line.");

        let g_num: u32 = match g_num.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match g_num.cmp(&num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => {
                println!("Too big!");
            }
        }
    }
}
