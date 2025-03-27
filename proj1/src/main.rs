// use rand::Rng;

fn main() {
    println!("Hello, world!");
    let name = String::from("LCH");

    println!("Hello, {name} !");

    let number = 1;
    println!("{}", number);

    let number2: u32 = 13;
    println!("{}", number2);

    let number2 = 13f64;
    println!("{}", number2);

    let mut num = {
        println!("Hello world");
        1
    };
    loop {
        println!("{}", num);
        num += 1;

        if num > 10 {
            break;
        }
    }

    panic!("hello");

    let str = String::from("123");
    let n = match str.parse::<u32>() {
        Ok(parsed_number) => {
            println!("parsed_number = {}", parsed_number);
            parsed_number
        }
        Err(_) => {
            println!("Not a number");
            0
        }
    };

    println!("n = {}", n);
}
