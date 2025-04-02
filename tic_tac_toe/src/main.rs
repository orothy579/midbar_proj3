use std::io;

struct Player {
    name: String,
    win: u8,
    position: Position,
}

struct Position {
    row: i32,
    col: i32,
}

enum Turn {
    Player1,
    Player2,
}

fn main() {
    println!("Let's play Tic-Tac-Toe !\n");

    let mut player1 = Player {
        name: String::from("player1"),
        win: 0,
        position: Position { row: 0, col: 0 },
    };

    let mut player2 = Player {
        name: String::from("player2"),
        win: 0,
        position: Position { row: 0, col: 0 },
    };

    'outer: loop {
        let mut board = [[' '; 3]; 3];
        let mut turn = Turn::Player1;

        // 게임
        'game: loop {
            // 1.보드 출력
            print_board(&board);

            // 이게 뭐임? 왜 필요한 거임? => current_player와 turn을 구분해줬다.
            let current_player = match turn {
                Turn::Player1 => &mut player1,
                Turn::Player2 => &mut player2,
            };

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

            // 이걸 왜 저장 해둬야 하나? 매개변수로 바로 pos를 넣어주면 안되는 이유는?
            let pos = Position { row, col };

            // 3. 사용자 입력 vaildation check
            if position_is_valid(&board, &pos) == false {
                println!("Wrong position.. Try again.\n");
                continue;
            }

            current_player.position = Position { row, col };

            // 4. 유효한 경우 보드에 마커 그리기
            board[row as usize][col as usize] = mark_of(&turn);

            // 5. 승리 조건 체크
            if is_win(&board) == true {
                print_board(&board);
                println!("{} is win!", current_player.name);
                break;
            }

            // 6. 무승부 체크
            if is_draw(&board) == true {
                print_board(&board);
                println!("Draw!");
                break;
            }

            // 7. 플레이어 교체
            turn = match turn {
                Turn::Player1 => Turn::Player2,
                Turn::Player2 => Turn::Player1,
            };
        }

        println!("Want to play again? (y/n): ");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line.");
        let answer = answer.trim();

        match answer {
            "y" | "Y" => {
                continue 'outer;
            }

            _ => {
                println!("Game Over!");
                break;
            }
        }
    }
    // 초기화
}

fn print_board(board: &[[char; 3]; 3]) {
    for b_row in board {
        for e in b_row {
            print!("[{e}]");
        }
        println!();
    }
}

fn position_is_valid(b: &[[char; 3]; 3], p: &Position) -> bool {
    if p.col < 0 || p.col > 2 || p.row < 0 || p.row > 2 {
        println!("Out of index..");
        return false;
    }
    //  이미 있는 포지션에 접근하는 경우 -> false
    if b[p.row as usize][p.col as usize] != ' ' {
        println!("\nAlready possesed..");
        return false;
    }

    return true;
}

fn mark_of(turn: &Turn) -> char {
    // if board is full

    match turn {
        Turn::Player1 => return 'X',
        Turn::Player2 => return 'O',
    }
}

fn is_win(board: &[[char; 3]; 3]) -> bool {
    let win_condition = [
        [board[0][0], board[1][0], board[2][0]],
        [board[0][1], board[1][1], board[2][1]],
        [board[0][2], board[1][2], board[2][2]],
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

fn is_draw(board: &[[char; 3]; 3]) -> bool {
    for b_row in board {
        for e in b_row {
            if *e == ' ' {
                return false;
            }
        }
    }
    return true;
}
