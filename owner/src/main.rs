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
fn main() {}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
