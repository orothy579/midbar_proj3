use rand::Rng;
use std::io;

// guessing game
// 1. random 값 생성
// 2. 사용자 입력 받기
// 3. 사용자 입력과 random 값 비교
//  3-1. 작으면 작다고 하기
//  3-2. 크면 크다고 하기
//  3-3. 같으면 같다고 하고 종료.
//
fn main() {
    // 1. random 값 생성
    let num = rand::rng().random_range(1..100);
    println!("{}", num);

    // 2. 사용자 입력 받기
    let mut guess = std::io::stdin();

    println!("guess number: {}", guess);

    // 3. 사용자 입력과 random 값 비교
}
