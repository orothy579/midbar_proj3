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
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky Penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

//Quarter variable가 UsState 값도 담고 있는 coin 열거형
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// Option<T> 을 match에 적용하기
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("{:?}", six);
//     println!("{:?}", none);
// }

// _ 자리 표시자 + 포괄 패턴
// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         6 => remove_fancy_hat(),
//         _ => (),
//     }
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}

// 어떤 값이 some 일 때만 코드를 실행하도록 하는 match
// fn main() {
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => println!("max is {}", max),
//         _ => (), // 보일러 플레이트 코드
//     }
// }

//if let을 사용한 간결한 제어 흐름 -> 한 패턴에 매칠될 때만 코드를 실행하고 다른 경우는 무시하는 match 문 작성시 사용
// fn main() {
//     let config_max = Some(3u8);

//     match config_max {
//         Some(max) => println!("The max is {}", max),
//         _ => (),
//     }

//     if let Some(max) = config_max {
//         println!("The max is {}", max);
//     }

//     let mut count = 0;
//     match coin {
//         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//         _ => count += 1;
//     }

//     let mut count = 0;
//     if let Coin::Quarter(state) = coin {
//         println!("State quarter from {:?}!", state);
//     } else {
//         count +=1;
//     }
// }

fn main() {}
