use std::io;

#[derive(Debug)]
enum MarkType {
    Nothing,
    Cross,
    Circle
}

fn main() {
    println!(" /////////////////////////////////////// TIC TAC TOE ///////////////////////////////////////
======================================== How to play ========================================
To mark a space with an X or an O type the space position following this format: RxC
Replace R with the row number and C with the column number, both starting at 1.
Type 'play' to play or 'quit' to quit.");

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read the line.");

        let command = command.trim();

        match command {
            "play" => break start_new_game(),
            "quit" => break,
            _ => println!("Type 'play' to play or 'quit' to quit."),
        };
    }
}

fn start_new_game() {
    let mut board = vec![
        MarkType::Nothing, MarkType::Nothing, MarkType::Nothing,
        MarkType::Nothing, MarkType::Nothing, MarkType::Nothing,
        MarkType::Nothing, MarkType::Nothing, MarkType::Nothing
    ];

    let mut current_turn = MarkType::Cross;

    draw_board(&board);
    show_turn(&current_turn);
    listen_for_input(&mut board, &mut current_turn);
}

fn draw_board(board: &[MarkType]) {
    for (i, _item) in board.iter().enumerate().step_by(3) {
        println!("[{}|{}|{}]", get_char_from_mark(&board[i]), get_char_from_mark(&board[i+1]), get_char_from_mark(&board[i+2]));
    }
}

fn show_turn(turn: &MarkType) {
    match turn {
        MarkType::Circle => println!("It's O turn!"),
        MarkType::Cross => println!("It's X turn!"),
        _ => ()
    }
}

fn get_char_from_mark(mark: &MarkType) -> char {
    match mark {
        MarkType::Nothing => ' ',
        MarkType::Circle => 'O',
        MarkType::Cross => 'X',
    }
}

fn set_cell(x: u8, y: u8, board: &mut Vec<MarkType>, new_cell: &MarkType) -> bool {
    let index = (y * 3 + x) as usize;
    match board[index] {
        MarkType::Nothing => {
            board[index] = match new_cell {
                MarkType::Cross => MarkType::Cross,
                MarkType::Circle => MarkType::Circle,
                MarkType::Nothing => MarkType::Nothing,
            };
            true
        },
        _ => false,
    }
}

fn check_winners(board: &[MarkType]) -> MarkType {
    let mut nboard: Vec<u8> = Vec::new();

    for item in board.iter() {
        nboard.push(match item {
            MarkType::Nothing => 0,
            MarkType::Cross => 1,
            MarkType::Circle => 2,
        });
    }

    // horizontal
    for i in 0..3 {
        let index = i * 3;
        if nboard[index] == nboard[index+1] && nboard[index] == nboard[index+2] {
            match &nboard[index] {
                1 => return MarkType::Cross,
                2 => return MarkType::Circle,
                _ => return MarkType::Nothing,
            };
        }
    }

    // vertical
    for i in 0..3 {
        if nboard[i] == nboard[i+3] && nboard[i] == nboard[i+6] {
            match &nboard[i] {
                1 => return MarkType::Cross,
                2 => return MarkType::Circle,
                _ => return MarkType::Nothing,
            };
        }
    }

    // corners
    if (nboard[0] == nboard[4] && nboard[0] == nboard[8])
        || (nboard[2] == nboard[4] && nboard[2] == nboard[6]) {
        match &nboard[4] {
            1 => return MarkType::Cross,
            2 => return MarkType::Circle,
            _ => return MarkType::Nothing,
        };
    }

    MarkType::Nothing
}

fn check_tie(board: &[MarkType]) -> bool {
    let mut used = 0;
    for item in board.iter() {
        used += match item {
            MarkType::Nothing => 0,
            _ => 1,
        };
    }
    used == 9
}

fn show_play_again(winner: MarkType) {
    match winner {
        MarkType::Circle => println!("O won!"),
        MarkType::Cross => println!("X won!"),
        MarkType::Nothing => println!("Draw!"),
    };

    println!("Type 'play' to play again or 'quit' to quit.");
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read the line.");

        let command = command.trim();
        match command {
            "play" => break start_new_game(),
            "quit" => break,
            _ => println!("Type 'play' to play again or 'quit' to quit."),
        };
    }
}

fn change_turn(turn: &mut MarkType) {
    *turn = match turn {
        MarkType::Circle => MarkType::Cross,
        MarkType::Cross => MarkType::Circle,
        _ => MarkType::Nothing,
    }
}

fn listen_for_input(board: &mut Vec<MarkType>, turn: &mut MarkType) {
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read the line.");

        let user_input: Vec<&str> = command.split('x').collect();

        if user_input.len() != 2 {
            println!("Try again. Format: RxC");
            continue;
        }
        else {
            let x: u8 = match user_input[1].trim().parse() {
                Ok(num) => num,
                Err(_e) => {
                    println!("Try again. Format: RxC");
                    continue;
                }
            };

            let y: u8 = match user_input[0].trim().parse() {
                Ok(num) => num,
                Err(_e) => {
                    println!("Try again. Format: RxC");
                    continue;
                }
            };
            
            if (x < 1 || x > 3) || (y < 1 || y > 3) {
                println!("Invalid {}! Try again. Format: RxC",
                    if x < 1 || x > 3 {
                            "column"
                    } else {
                        "row"
                    }
                );
                continue;
            }

            let has_set = set_cell(x-1, y-1, board, turn);
            if has_set {
                draw_board(board);

                let winner = check_winners(board);
                if let MarkType::Nothing = winner {
                    if check_tie(board) {
                        break show_play_again(winner);
                    }
                    else {
                        change_turn(turn);
                        show_turn(turn);
                    }
                }
                else {
                    break show_play_again(winner);
                }
            }
            else {
                println!("That space has been marked already, try again. Format: RxC");
                continue;
            }
        }
    }
}