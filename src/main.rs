use std::io;
use std::cmp::Ordering;

#[derive(Debug)]
enum MarkType {
    Nothing,
    Cross,
    Circle
}

fn main() {
    println!("Bem-vindo ao jogo da velha!
==================== Como jogar ====================
Para marcar um X ou O digite
o numero da célula no seguinte formato:
LxC
Troque L pelo número da linha e C pelo número da coluna, ambos começando em 1.
Digite 'jogar' para jogar ou 'sair' para sair.");

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Falha ao ler a linha");

        let command = command.trim();

        if command == "jogar" {
            break start_new_game();
        }
        else if command == "sair" {
            break;
        }
        else {
            println!("Digite 'jogar' para jogar ou 'sair' para sair.");
        }
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

fn draw_board(board: &Vec<MarkType>) {
    println!("[{}|{}|{}]", get_char_from_mark(&board[0]), get_char_from_mark(&board[1]), get_char_from_mark(&board[2]));
    println!("[{}|{}|{}]", get_char_from_mark(&board[3]), get_char_from_mark(&board[4]), get_char_from_mark(&board[5]));
    println!("[{}|{}|{}]", get_char_from_mark(&board[6]), get_char_from_mark(&board[7]), get_char_from_mark(&board[8]));
}

fn show_turn(turn: &MarkType) {
    match turn {
        MarkType::Circle => println!("É a vez do círculo!"),
        MarkType::Cross => println!("É a vez da cruz!"),
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

fn check_winners(board: &Vec<MarkType>) -> MarkType {
    let mut nboard: Vec<u8> = Vec::new();

    for item in board.iter() {
        nboard.push(match item {
            MarkType::Nothing => 0,
            MarkType::Cross => 1,
            MarkType::Circle => 2,
        });
    }

    // horizontal
    for (i, item) in nboard.iter().enumerate().step_by(3) {
        if item == &nboard[i+1] && item == &nboard[i+2] {
            match item {
                1 => return MarkType::Cross,
                2 => return MarkType::Circle,
                _ => return MarkType::Nothing,
            };
        }
    }

    // vertical
    for i in 0..3 {
        if &nboard[i] == &nboard[i+3] && &nboard[i] == &nboard[i+6] {
            match &nboard[i] {
                1 => return MarkType::Cross,
                2 => return MarkType::Circle,
                _ => return MarkType::Nothing,
            };
        }
    }

    // corners
    if (&nboard[0] == &nboard[4] && &nboard[0] == &nboard[8])
        || (&nboard[2] == &nboard[4] && &nboard[2] == &nboard[6]) {
        match &nboard[4] {
            1 => return MarkType::Cross,
            2 => return MarkType::Circle,
            _ => return MarkType::Nothing,
        };
    }

    MarkType::Nothing
}

fn check_tie(board: &Vec<MarkType>) -> bool {
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
        MarkType::Circle => println!("Círculo venceu!"),
        MarkType::Cross => println!("Cruz venceu!"),
        MarkType::Nothing => println!("Empate!"),
    };

    println!("Digite 'jogar' para jogar novamente ou 'sair' para sair.");
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Falha ao ler a linha.");

        let command = command.trim();
        if command == "jogar" {
            break start_new_game();
        }
        else if command == "sair" {
            break;
        }

        println!("Digite 'jogar' para jogar novamente ou 'sair' para sair.");
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
        io::stdin().read_line(&mut command).expect("Falha ao ler a linha.");

        let user_input: Vec<&str> = command.split("x").collect();

        if user_input.len() != 2 {
            println!("Tente novamente. Formato: LxC");
            continue;
        }
        else {
            let x: u8 = match user_input[1].trim().parse() {
                Ok(num) => num,
                Err(_e) => {
                    println!("Tente novamente. Formato: LxC");
                    continue;
                }
            };

            let y: u8 = match user_input[0].trim().parse() {
                Ok(num) => num,
                Err(_e) => {
                    println!("Tente novamente. Formato: LxC");
                    continue;
                }
            };
            
            if let Ordering::Greater = x.cmp(&3) {
                println!("Coluna inválida! Tente novamente. Formato: LxC");
                continue;
            }

            if let Ordering::Less = x.cmp(&1) {
                println!("Coluna inválida! Tente novamente. Formato: LxC");
                continue;
            }

            if let Ordering::Greater = y.cmp(&3) {
                println!("Linha inválida! Tente novamente. Formato: LxC");
                continue;
            }

            if let Ordering::Less = y.cmp(&1) {
                println!("Linha inválida! Tente novamente. Formato: LxC");
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
                println!("Essa célula já foi marcada, tente novamente. Formato: LxC");
                continue;
            }
        }
    }
}