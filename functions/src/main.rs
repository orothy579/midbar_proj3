fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("y is {y}");
    let z = five();
    println!("z is {z}");

    let a = plus_one(z);

    println!("a is {a}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
