/*
Fahrenheit to celcious
*/

use std::io;

fn main() {
    println!("Please input Fahrenheit :  ");
    let mut f = String::new();
    let c: f64;

    io::stdin().read_line(&mut f).expect("failed to read line.");
    let f: f64 = f.trim().parse().expect("failed to convert");

    c = (f - 32.0) * 5.0 / 9.0;
    println!("celcious is {c}");
}
