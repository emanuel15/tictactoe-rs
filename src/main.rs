use std::io;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum MarkType {
    Nothing,
    Cross,
    Circle
}

use MarkType::*;

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
        Nothing, Nothing, Nothing,
        Nothing, Nothing, Nothing,
        Nothing, Nothing, Nothing
    ];

    let mut current_turn = Cross;

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
        Circle => println!("It's O turn!"),
        Cross => println!("It's X turn!"),
        _ => ()
    }
}

fn get_char_from_mark(mark: &MarkType) -> char {
    match mark {
        Nothing => ' ',
        Circle => 'O',
        Cross => 'X',
    }
}

fn set_cell(x: u8, y: u8, board: &mut Vec<MarkType>, new_cell: &MarkType) -> bool {
    let index = (y * 3 + x) as usize;
    match board[index] {
        Nothing => {
            board[index] = match new_cell {
                Cross => Cross,
                Circle => Circle,
                Nothing => Nothing,
            };
            true
        },
        _ => false,
    }
}

fn check_winners(board: &[MarkType]) -> MarkType {

    let winner_positions = vec![
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6]
    ];

    for i in 0..8 {
        if board[winner_positions[i][0]] == board[winner_positions[i][1]] && board[winner_positions[i][0]] == board[winner_positions[i][2]] {
            return board[winner_positions[i][0]].clone();
        }
    }

    Nothing
}

fn check_tie(board: &[MarkType]) -> bool {
    let mut used = 0;
    for item in board.iter() {
        used += match item {
            Nothing => 0,
            _ => 1,
        };
    }
    used == 9
}

fn show_play_again(winner: MarkType) {
    match winner {
        Circle => println!("O won!"),
        Cross => println!("X won!"),
        Nothing => println!("Draw!"),
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
        Circle => Cross,
        Cross => Circle,
        _ => Nothing,
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