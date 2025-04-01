fn main() {
    //move
    let s = String::from("Hello");

    let s1 = s;
    println!("{s1}");

    //copy
    // let i = 9;
    // let q = i;
    // println!("{q}");

    //대여
    // let mut b = 10;
    // let copy_b = copy_function(b);
    // println!("b is {b}");
    // println!("modified b is {copy_b}");

    // let borrow_b = borrow_function(&mut b);
    // println!("b is {b}");
    // println!("modified b is {borrow_b}");

    // 함수 없이 참조하기
    // let mut str = String::from("hello");

    // {
    //     let str1 = &str;
    //     let str2 = &str;

    //     println!("{} , {}", str1, str2);

    //     let str3 = &mut str;
    //     str3.push_str(", world");
    //     println!("{str3} ,{str1}");
    // }
}

// fn copy_function(mut b1: i32) -> i32 {
//     b1 = b1 + 1;
//     println!("{}", b1);
//     b1
// }

// fn borrow_function(b1: &mut i32) -> i32 {
//     *b1 = *b1 + 1;
//     println!("{}", *b1);
//     *b1
// }
