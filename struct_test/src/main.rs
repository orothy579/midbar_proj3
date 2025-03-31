// user 구조체 정의
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");

    // user 구조체 인스턴스 생성
    let user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("email@email.com"),
        sign_in_count: 11,
    };

    // user 인스턴스의 email 필드값 변경
    let mut user2 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("email@email.com"),
        sign_in_count: 11,
    };

    user2.email = String::from("anotheremail@email.com");

    let fn_user = build_user(String::from("hello"), String::from("bello"));

    println!("{}", fn_user.email);

    // 구조체 업데이트 문법
    //기존 인스턴스를 이용해 새 인스턴스를 만들기

    // user1의 값 중 하나를 다르게 한 새로운 User 인스턴스 생성하기
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // user1의 username 필드의 String이 user3로 이동(move)했기 때문에 에러 발생
    // let user3 = User {
    //     email: String::from("anotheranthoer@example.com"),
    //     ..user1
    // }

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    // tuple struct : 이름없이 타입만 적은 형태
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 10, 0);
    let origin = Point(0, 0, 10);

    let (r, g, b) = (black.0, black.1, black.2);

    println!("{}, {}, {}", r, g, b);
}

// 사용자의 email과 name을 전달받고 User 인스턴스를 반환하는 build_user 함수
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// 변수명과 필드명이 같은 username, email 필드 초기화 축양법을 적용한 함수
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
