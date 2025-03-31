// n 번째 피보나치 수 생성하기
// use std::io;
// fn main() {
//     let mut n = String::new();

//     io::stdin()
//         .read_line(&mut n)
//         .expect("should be read line to get numbers");
//     let n: u32 = match n.trim().parse() {
//         Ok(n) => n,
//         Err(_) => {
//             println!("not a number");
//             0
//         }
//     };

//     let res = fibo(n);
//     println!("n번째 피포나치 수열은 {res}입니다.");
// }

// fn fibo(x: u32) -> u32 {
//     let mut res = 0;
//     if x > 1 {
//         res = fibo(x - 2) + fibo(x - 1);
//         return res;
//     } else if x == 1 {
//         res += 1;
//         return res;
//     } else {
//         return res;
//     }
// }

// fn fibo(x: u32) -> u32 {
//     if x == 0 {
//         return 0;
//     }

//     let mut dp = vec![0; (x + 1) as usize];
//     dp[0] = 0;
//     dp[1] = 1;

//     for i in 2..=x as usize {
//         dp[i] = dp[i - 1] + dp[i - 2];
//     }

//     dp[x as usize]
// }

// fn fibo(x: u32) -> u32 {
//     let res = 0;
//     if x == 0 {
//         return res;
//     } else if x == 1 {
//         return res + 1;
//     } else {
//         return fibo(x - 1) + fibo(x - 2);
//     }
// }

fn main() {
    println!("hello ");
}
