use std::io;

#[derive(PartialEq, Clone)]
struct Player {
    name: String,
    win: u8,
    position: Position,
}

#[derive(PartialEq, Clone)]
struct Position {
    row: u8,
    col: u8,
}

fn main() {
    println!("Hello, world!");

    let player1 = Player {
        name: String::from("player1"),
        win: 0,
        position: Position { row: 0, col: 0 },
    };

    let player2 = Player {
        name: String::from("player2"),
        win: 0,
        position: Position { row: 0, col: 0 },
    };

    // 초기화
    let mut current_player = player1.clone();
    let mut board: [[&str; 3]; 3] = [[" "; 3]; 3];

    // 게임
    'main_loop: loop {
        // 1.보드 출력
        print_board(&board);
        // 2. 사용자 입력

        let mut row = String::new();
        let mut col = String::new();

        println!("Please input the row number.");
        io::stdin()
            .read_line(&mut row)
            .expect("Failed to read line");

        println!("Please input the column number.");
        io::stdin()
            .read_line(&mut col)
            .expect("Failed to read line");

        let row: usize = match row.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let col: usize = match col.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // // 3. 사용자 입력 vaildation check
        // if position_is_valid(&board, &current_player.position) == false {
        //     println!("Wrong position.. Try again.");
        //     continue;
        // }

        // // 4. 유효한 경우 보드에 마커 그리기
        // board[row][col] = mark_of(&current_player);

        // // 5. 승리 조건 체크
        // if is_win() {
        //     println!("You win!");
        //     break;
        // }

        // // 6. 무승부 체크
        // if is_draw() {
        //     println!("draw!");
        //     break;
        // }

        // 7. 플레이어 교체
        if current_player == player1 {
            current_player = player2.clone();
        } else {
            current_player = player1.clone();
        }
    }
}

fn print_board(board: &[[&str; 3]; 3]) {
    println!("{:#?}", board);
}

fn position_is_valid(b: &String, p: &Position) -> bool {
    return false;
}

fn mark_of(p: &Player) {}

fn is_win() -> bool {
    return true;
}

fn is_draw() -> bool {
    return true;
}
