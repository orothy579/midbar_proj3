struct Player {
    name: String,
    win: u8,
}

fn main() {
    println!("Hello, world!");
    let player1 = Player {
        name: String::from("player1"),
        win: 0,
    };

    let player2 = Player {
        name: String::from("player2"),
        win: 0,
    };

    let current_player = player1;
}
