// use std::slice::Windows;

// // 구조체 정의
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// // 구조체 사용
// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("somthing"),
//         email: String::from("some@example.com"),
//         sign_in_count: 1,
//     };
// }

// // 가변 인스턴스의 email 필드 값 변경
// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("some"),
//         email: String::from("some@example.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("another@example.com");
// }

// // 사용자의 이메일과 이름을 전달받고 User 인스턴스를 반환하는 build_user 함수
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// // 필드 초기화 축약법 사용
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// // 구조체 업데이트 문법
// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("some"),
//         email: String::from("some@example.com"),
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };

//     // 새로운 email 값으로 User 구조체의 인스턴스 생성, 나머지 필드는 구조체 업데이트 문법으로 user1의 필드 값을 사용하기
//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
// }

// // tuple 구조체
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// // 필드가 없는 유사 유닛 구조체
// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
// }

// // 사각형 넓이를 계산하는 프로그램

// //각 변수에 지정된 너비와 높이로 사각형 넓이 계산하기
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!("The area is {}", area(width1, height1));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// // 사각형의 너비와 높이를 튜플로 명시하는 코드
// fn main() {
//     let rect1 = (30, 50);
//     println!("The area of the rectangle is {}", area(rect1));
// }

// fn area(dims: (u32, u32)) -> u32 {
//     dims.0 * dims.1
// }

// Rectangle 구조체 정의
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("The area of the rectangle is {}", area(&rect1));
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// Rectangle instance 내 모든 필드 값 출력하기

// // #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {}", rect1);
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: 30 * scale,
//         height: 50,
//     };

//     dbg!(&rect1);
// }

// 메서드 정의 하기
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(self: &Self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     )
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn width(self: &Self) -> bool {
//         self.width > 0
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("The rectangle has {}", rect1.width);
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(self: &Self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(self: &Self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }

//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }

// // 더 많은 매개변수를 가진 메서드
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };

//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }

// enum IpAddrKind {
//     V4,
//     V6, // variant
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     route(IpAddrKind::V4);
// }

// fn route(ip_kind: IpAddrKind) {}

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// enum 과 struct의 차이

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);

// fn main() {
//     let some_number = Some(5);
//     let some_char = Some('e');
//     let absent_number: Option<i32> = None;

// }

// match 제어 흐름 구조

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
