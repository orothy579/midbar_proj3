// enum IpAddrKind {
//     V4,
//     V6,
// }

// fn main() {
//     println!("Hello, world!");

//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }

// fn route(ip_kind: IpAddrKind) {}

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(String::from("127..0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// enum Message {
//     // 연관된 데이터가 없습니다.
//     Quit,
//     // 구조체처럼 이름이 있는 필드를 갖습니다.
//     Move { x: i32, y: i32 },
//     // 하나의 String을 갖습니다.
//     Write(String),
//     // 세 개의 i32를 갖습니다.
//     ChangeColor(i32, i32, i32),
// }

// impl Messgae {
//     fn call(&self) {}
// }

// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// struct로 같은 역할하는 데이터 타입 정의
// struct QuitMessage; // unit 구조체
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }

// struct WriteMessage(String); // 튜플 struct
// struct ChangeColorMessage(i32, i32, i32); // 튜플 struct

//Option<T> 열거형
// fn main() {
//     let some_number = Some(5);
//     let some_char = Some('e');

//     let absent_number: Option<i32> = None;

//     // compile error
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);

//     //compile이 안되게 하는 것이 Option<T>의 목적
//     let sum = x + y;
// }

// match 제어 흐름 구조
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}
