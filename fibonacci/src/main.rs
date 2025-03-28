// n 번째 피보나치 수 생성하기

fn main() {
    let n = 10;
    let res = fibo(n);
    println!("n번째 피포나치 수열은 {res}입니다.");
}

fn fibo(x: u32) -> u32 {
    let mut res = 0;
    if x > 1 {
        res = fibo(x - 2) + fibo(x - 1);
        return res;
    } else if x == 1 {
        res += 1;
        return res;
    } else {
        return res;
    }
}
