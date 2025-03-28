fn main() {
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scopr is {y}");
    }

    println!("The value of y is {y}");

    //shadowing
    let spaces = "     ";
    let spaces = spaces.len();
    println!("{spaces}");

    let mut spaces1 = "      ";
    let spaces1 = spaces1.len();

    let guess: u32 = "42".parse().expect("Not a number ");
    println!("{guess}");

    let z = 2.3;
    let q: f32 = 3.4;
    // let w: u8 = 256;
    println!("z: {z}, q: {q}");

    let sum = 5 + 10;
    println!("sum :  {sum}");

    let diff = 95.5 - 3.4;
    println!("diff : {diff}");

    let prod = 4 * 30;
    println!("prod : {prod}");

    let quo = 56.7 / 32.2;
    println!("quo : {quo}");

    let tranc = -5 / 3;
    println!("tranc : {tranc}");

    let remainder = 43 % 5;
    println!("ramainder : {remainder}");

    let t = true;
    let f = false;

    println!("true : {t}");

    let tup: (i32, f64, u8) = (5000, 6.3, 1);
    let (x, y, z) = tup;
    println!("x : {x} , y : {y}, z : {z}");
    let tup_0 = tup.0;
    let tup_1 = tup.1;
    let tup_2 = tup.2;

    println!("tup.0 : {tup_0} , tup.1 : {tup_1} , tup.2 : {tup_2}");

    let a = [1, 2, 3, 4, 5];
}
