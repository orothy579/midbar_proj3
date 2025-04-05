#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sigin_in_count: u64,
}

impl User {
    fn some(&self) {
        println!("{}", self.active);
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

enum IpAddrKind {
    V4(String),
    V6(u8, u8, u8, u8),
}

enum msg {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

struct QuitMsg;
struct MoveMsg {
    x: i32,
    y: i32,
}

// enum Option<T> {
//     None,
//     some(T),
// }

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("some"),
        email: String::from("some@example.com"),
        sigin_in_count: 1,
    };

    println!("The active is {}", user1.active);

    user1.set_active(false);

    println!("The active is {}", user1.active);

    println!("{:#?}", user1);

    let addr = IpAddrKind::V4(String::from("127.0.0.1"));

    println!("hi");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
