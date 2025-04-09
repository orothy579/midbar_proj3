use std::io::{self, Read};

struct Player {
    id: Id,
    name: String,
    win: u8,
    marker: char,
}

struct Position {
    row: i8,
    col: i8,
}

enum Id {
    Player1,
    Player2,
}

fn main() {
    println!("Let's play Tic-Tac-Toe !\n");

    let mut player1 = Player {
        id: Id::Player1,
        name: String::from("player1"),
        win: 0,
        marker: 'X',
    };

    let mut player2 = Player {
        id: Id::Player2,
        name: String::from("player2"),
        win: 0,
        marker: 'O',
    };

    'outer: loop {
        let mut board = [[' '; 3]; 3];
        let mut current_player = &mut player1;

        // 게임
        'game: loop {
            // 1.보드 출력
            print_board(&board);

            println!("{}'s turn.", current_player.name);

            // 2. 사용자 입력
            let mut row = String::new();
            let mut col = String::new();

            let mut input = String::new();
            println!("Please input the row, col number. (1~3 or 'q' to quit)");

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");

            let cell: Vec<i32> = input
                .split_whitespace()
                .map(|x| x.parse::<i32>().expect("Failed to parse"))
                .collect();

            println!("cell : {:?}", cell);

            println!("Please input the row number (1~3, or 'q' to quit):");
            io::stdin()
                .read_line(&mut row)
                .expect("Failed to read line");

            if row.trim() == "q" {
                println!("Quiting the game..");
                break 'game;
            }

            println!("Please input the column number (1~3, or 'q' to quit):");
            io::stdin()
                .read_line(&mut col)
                .expect("Failed to read line");

            if col.trim() == "q" {
                println!("Quiting the game..");
                break 'game;
            }

            let row: i8 = match row.trim().parse() {
                Ok(num) => num,
                Err(_) => continue 'game,
            };

            let col: i8 = match col.trim().parse() {
                Ok(num) => num,
                Err(_) => continue 'game,
            };

            let pos = Position { row, col };

            // 3. 사용자 입력 vaildation check
            if position_is_valid(&board, &pos) == false {
                println!("Wrong position.. Try again.\n");
                continue 'game;
            }

            // 현재 플레이어 position 업데이트
            // current_player.position = pos;

            // 4. 유효한 경우 보드에 마커 그리기

            board[row as usize][col as usize] = current_player.marker;

            // 5. 승리 조건 체크
            if is_win(&board) {
                print_board(&board);
                println!("{} win!", current_player.name);
                current_player.win += 1;

                println!(
                    "Current score => {}: {}, {}: {}",
                    player1.name, player1.win, player2.name, player2.win
                );
                break 'game;
            }

            // 6. 무승부 체크
            if is_draw(&board) {
                print_board(&board);
                println!("Draw!");
                break 'game;
            }

            // 7. 플레이어 교체

            current_player = match current_player.id {
                Id::Player1 => &mut player2,
                Id::Player2 => &mut player1,
            };
        }

        // 8. 다음판 진행 여부 묻기
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
                break 'outer;
            }
        }
    }

    println!(
        "Final Score => {}: {}, {}: {}",
        player1.name, player1.win, player2.name, player2.win
    );
    println!("Game Over!");
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

fn position_is_valid(board: &[[char; 3]; 3], p: &Position) -> bool {
    if p.col < 1 || p.col > 3 || p.row < 1 || p.row > 3 {
        println!("Out of index..");
        return false;
    }
    //  이미 있는 포지션에 접근하는 경우 -> false
    if board[p.row as usize][p.col as usize] != ' ' {
        println!("\nAlready occupied..");
        return false;
    }

    return true;
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
