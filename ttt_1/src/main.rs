use std::io;
struct Player {
    name: String,
    win: u8,
    position: Position,
}

struct Position {
    row: i8,
    col: i8,
}

enum Turn {
    Player1,
    Player2,
}

fn main() {
    println!("Hello, world!");

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

    //loop
    'outer: loop {
        //init
        let mut board = [[' '; 3]; 3];
        let mut turn = Turn::Player1;

        'game: loop {
            // 1.보드 출력
            print_board(&board);

            let current_player = match turn {
                Turn::Player1 => &mut player1,
                Turn::Player2 => &mut player2,
            };

            let mut row = String::new();
            let mut col = String::new();

            // 2.사용자 입력
            println!("Please input row");
            io::stdin()
                .read_line(&mut row)
                .expect("Failed to read line.");

            println!("Please input col");
            io::stdin()
                .read_line(&mut col)
                .expect("Failed to read line.");

            let row: i8 = row.trim().parse().expect("Failed to parse");
            let col: i8 = col.trim().parse().expect("Failed to parse");

            let pos = Position { row, col };

            // 3. 사용자 validation check
            // 3-1. 유효한 범위 내에 있는지 확인
            // 3-2. 음수인지 확인
            if position_is_valid(&board, &pos) == false {
                println!("Wrong position! try again");
                continue 'game;
            }

            current_player.position = pos;

            // 4.maker 그리기
            board[row as usize][col as usize] = mark_of(&turn);

            // 5.win check
            if is_win(&board) {
                println!("{} win!", current_player.name);
                current_player.win += 1;
                break 'game;
            }

            // 6.draw check
            if is_draw(&board) {
                println!("Draw");
                break 'game;
            }
            // 7.turn change
            turn = match turn {
                Turn::Player1 => Turn::Player2,
                Turn::Player2 => Turn::Player1,
            }
        }

        // 8.게임 다시 할건지 묻기

        let mut answer = String::new();
        println!("Want to play the game again?");
        io::stdin()
            .read_line(&mut answer)
            .expect("failed to read lines");

        let answer = answer.trim();

        match answer {
            "y" | "Y" => continue 'outer,
            _ => {
                println!(
                    "Final score {}: {}, {}: {}",
                    player1.name, player1.win, player2.name, player2.win
                );
                break 'outer;
            }
        }
    }
}

fn print_board(b: &[[char; 3]; 3]) {
    for r in b {
        for c in r {
            print!("[{c}]");
        }
        println!();
    }
}

fn position_is_valid(b: &[[char; 3]; 3], p: &Position) -> bool {
    if p.col < 0 || p.row < 0 || p.col > 2 || p.row > 2 {
        return false;
    }

    if b[p.row as usize][p.col as usize] != ' ' {
        return false;
    }
    return true;
}

fn mark_of(t: &Turn) -> char {
    match t {
        Turn::Player1 => 'X',
        Turn::Player2 => 'O',
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

    for r in win_condition {
        if r == ['X'; 3] || r == ['O'; 3] {
            return true;
        }
    }

    return false;
}

fn is_draw(board: &[[char; 3]; 3]) -> bool {
    for r in board {
        for c in r {
            if *c == ' ' {
                return false;
            }
        }
    }

    return true;
}
