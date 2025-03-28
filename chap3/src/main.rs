use std::io;

fn main() {
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index.trim().parse().expect("Failed to parse");

    let n: [i32; 5] = [1, 2, 3, 4, 5];
    let result = n[index];

    println!("The number of index is : {result}");

    another_function();
}

fn another_function() {
    println!("Another function!");
}
