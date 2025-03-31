// fn main() {
//     let s1 = gives_ownership(); // 이 함수의 return value를 s1로 이동
//     let s2 = String::from("hello"); // s2가 스코프 안으로 들어옵니다.
//     let s3 = takes_and_gives_back(s2);

//     println!("s1 is {s1}");
//     println!("s2 is {s2}");
//     println!("s3 is {s3}");
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string

// // 예제 4.5
// fn main() {
//     let s1 = String::from("hello");
//     let (s2, len) = calculate_length(s1);
//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// // 예제 4-5-1
// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
//}

// // 예제 4-6
// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
//     println!("s is {s}");
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world")
// }

// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s; // no problem.
//     let r2 = &s; // no problem.
//     println!("{} {}", r1, r2);

//     let r3 = &mut s; // problem!

//     println!("{}", r3);
// }

// 예제 4-7
// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     s.clear();
//     println!("word is {word}");

//     // let hello = &s[0..5];
//     // let world = &s[6..11];

//     let ss = String::from("hello");

//     let len = ss.len();
//     let slice = &ss[3..len];
//     let slice1 = &ss[3..];

//     println!("slice is {slice}");
//     println!("slice1 is {slice1}");
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);

//     s.clear();

//     println!("the first word is : {}", word);
// }

// fn main() {
//     let my_string = String::from("hello world");
//     let word = first_word(&my_string[0..6]);
//     println!("word is {word}");
//     let word = first_word(&my_string[..]);
//     println!("word is {word}");
//     let word = first_word(&my_string);
//     println!("word is {word}");

//     let my_string_literal = "hello world";

//     let word = first_word(&my_string_literal[0..6]);
//     println!("word is {word}");

//     let word = first_word(&my_string_literal[..]);
//     println!("word is {word}");

//     // let word = first_word(my_string_literal);
//     // let word = first_word(&my_string);

//     let a = [1, 2, 3, 4, 5];

//     let slice = &a[1..3];

//     assert_eq!(slice, &[2, 3], "we are testing");
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn main() {
//     let mut s = String::from("hello");
//     s.push_str(", world");

//     println!("{s}");
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{s2}");
// }

// fn main() {
//     let s = String::from("hello");

//     {
//         println!("{}", s);
//     }
//     println!("{}", s);
//     ownership(s);
//     // println!("{}", s);

//     let x = 19;
//     makes_copy(x);
// }

// // takes_ownership
// fn ownership(some: String) {
//     println!("{}", some);
// }

// fn makes_copy(some_x: i32) {
//     println!("{some_x}");
// }

// fn main() {
//     let s1 = String::from("hello");
//     let (s2, len) = cal(s1);
//     println!("{}..{}", s1, len);
// }

// fn cal(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// fn main() {
//     let s1 = String::from("hello00");
//     let len = cal(&s1);
//     println!("the length of {s1} is {len}");
// }

// fn cal(some: &String) -> usize {
//     let len = some.len();
//     len
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);

//     println!("{s}");
// }

// fn change(some: &mut String) {
//     some.push_str(", world!");
// }

// fn main() {
//     let dan = dangle();
//     println!("{dan}");
// }

// fn dangle() -> String {
//     let s = String::from("hello");

//     s
// }

fn main() {
    let mut s = String::from("Hello world");
    let ss = "hi";

    let res = first_word(&mut s);
    println!("The index is {res}");
    println!("The index is {res}");

    s.clear(); // mutable borrow 발생
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
