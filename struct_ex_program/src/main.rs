// 각 변수에 지정된 너비와 높이로 사각형 넓이 계산하기
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//사각형의 너비와 높이를 튜플로 명시하는 코드 ==> code 내에 의미 전달 x , 어떤게 width 인지 height인지 모름!
// fn main() {
//     let rect1 = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area1(rect1)
//     );
// }

// fn area1(dim: (u32, u32)) -> u32 {
//     dim.0 * dim.1
// }

// rectangle 구조체 정의
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// 디버깅용으로 Rectangle 인스턴츠 출력, 외부 속성 Debug 트레이트 파생하기.
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );

//     // rectangle instance 내 모든 필드값 출력하기
//     println!("rect is {:#?} ", rect1);
// }

// // struct의 소유권을 가져와버리면, rect1을 area 호출 이후에 더이상 사용할 수 없으므로 referance type으로 정하여 borrowing.
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

// 메서드 정의하기
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// // 메서드 블록
// impl Rectangle {
//     //Self는 imple block 대상이 되는 타입의 별칭
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
//     );
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 필드와 중복된 이름의 매서드 정의
impl Rectangle {
    fn width(self: &Self) -> bool {
        self.width > 0
    }

    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn can_hold(self: &Self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // self를 첫번째 매개변수로 갖지 않는 연관 함수
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 연관함수 호출 시 ::
    let sqr = Rectangle::square(3);
    dbg!(&sqr);
}
