use std::io;
struct Player {
    name: String,
    win: u8,
    position: Position,
}

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
    let mut current_player = player1;
    let game_end = false;
    let mut board = String::from("");

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

        current_player.position.row = match row.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        current_player.position.col = match col.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 3. 사용자 입력 vaildation check
        if position_is_valid(&board, &current_player.position) == false {
            println!("Wrong position.. Try again.");
            continue;
        }

        // 4. 유효한 경우 보드에 마커 그리기

        // 5. 승리 조건 체크

        // 6. 무승부 체크

        // 7. 플레이어 교체
        if current_player == player1 {
            current_player = player2;
        } else {
            current_player = player1;
        }
    }
}

fn print_board(board: &String) {}

fn position_is_valid(b: &String, p: &Position) -> bool {
    return false;
}
