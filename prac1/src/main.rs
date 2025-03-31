fn main() {
    let mut x = 1;

    loop {
        println!("x is : {x}");

        x = x + 1;

        if x > 10 {
            break;
        };
    }
}
