use std::io;

#[derive(PartialEq, Clone)]
struct Player {
    name: String,
    win: u8,
    position: Position,
}

#[derive(PartialEq, Clone)]
struct Position {
    row: i32,
    col: i32,
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
    let mut board: [[char; 3]; 3] = [[' '; 3]; 3];

    // 게임
    loop {
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

        let row: i32 = match row.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let col: i32 = match col.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let pos = Position { row, col };

        // 3. 사용자 입력 vaildation check
        if position_is_valid(&board, &pos) == false {
            println!("Wrong position.. Try again.");
            continue;
        }

        current_player.position = Position { row, col };

        // 4. 유효한 경우 보드에 마커 그리기
        board[row as usize][col as usize] = mark_of(&current_player);

        // 5. 승리 조건 체크
        if is_win(&board) == true {
            println!("{:?}", board);
            println!("You win!");
            break;
        }

        // 6. 무승부 체크
        if is_draw(&board) {
            println!("draw!");
            break;
        }

        // 7. 플레이어 교체
        // clone 말고 다른 방법으로 교체 예정 -> player 교체가 안된다.
        if current_player == player1 {
            current_player = player2.clone();
        } else {
            current_player = player1.clone();
        }
    }
}

fn print_board(board: &[[char; 3]; 3]) {
    println!("{:?}", board);
}

// logic이 잘못된듯.. -> 한번 wrong positoin 걸리면 계속 wrong positoin 걸린다.
fn position_is_valid(b: &[[char; 3]; 3], p: &Position) -> bool {
    //  (-1, -1) 과 같이 음수에 접근하는 경우
    //     -> false -> Question: 이미 타입이 u8인데, 음수가 받아지나?
    if p.col < 0 || p.row > 2 {
        return false;
    }
    //  이미 있는 포지션에 접근하는 경우 -> false
    if b[p.row as usize][p.col as usize] != ' ' {
        return false;
    }

    return true;
}

fn mark_of(p: &Player) -> char {
    // if board is full
    if p.name == "player1" {
        return 'X';
    } else {
        return 'O';
    }
}

fn is_win(board: &[[char; 3]; 3]) -> bool {
    let win_condition = [
        [board[0][0], board[1][0], board[2][0]],
        [board[1][1], board[1][1], board[1][2]],
        [board[2][0], board[2][1], board[2][2]],
        [board[0][0], board[0][1], board[0][2]],
        [board[1][0], board[1][1], board[1][2]],
        [board[2][0], board[2][1], board[2][2]],
        [board[0][0], board[1][1], board[2][2]],
        [board[0][2], board[1][1], board[2][0]],
    ];

    for cond in win_condition {
        if cond == ['X'; 3] || cond == ['O'; 3] {
            return true;
        }
    }

    return false;
}

fn is_draw(b: &[[char; 3]; 3]) -> bool {
    for b_row in b {
        for e in b_row {
            if *e != ' ' {
                return false;
            }
        }
    }
    return true;
}
