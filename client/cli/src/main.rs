use std::io;
use strum_macros::EnumIter;
#[path = "../../src/toot_and_otto/player.rs"]
mod player;
use player::Player;

#[path = "../../src/connect4/connect4.rs"]
mod connect4;
use connect4::Connect4;

#[path = "../../src/connect4/piece_color.rs"]
mod piece_color;
use piece_color::PieceColor;

#[path = "../../src/connect4/cpu_con4.rs"]
mod cpu_con4;

#[path = "../../src/toot_and_otto/toot_and_otto.rs"]
mod toot_and_otto;
use toot_and_otto::TootAndOtto;

#[path = "../../src/toot_and_otto/piece_letter.rs"]
mod piece_letter;
use piece_letter::PieceLetter;

#[path = "../../src/toot_and_otto/cpu_toot.rs"]
mod cpu_toot;

// use piece_letter::PieceLetter;

// Gets input from the user and returns a usize and a tuple
fn get_input_connect4() -> (usize, bool) {
    // Gets input from the user, handles any errors
    let mut col = String::new();
    if let Err(_) = io::stdin().read_line(&mut col) {
        println!("Input failed, try again");
        return (0, false);
    };

    // Parses the users input to a usize
    let col: usize = match col.trim().parse() {
        Err(_) => {
            println!("Invalid Input, Try Again");
            return (0, false);
        }
        Ok(col) => col,
    };

    (col, true)
}

/// Gets input from the user and returns a usize and a tuple
fn get_input_toot_and_otto() -> (char, usize, bool) {
    // Gets input from the user, handles any errors
    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        println!("Input failed, try again");
        return ('X', 0, false);
    };

    // Splits the input
    let parameters: Vec<&str> = input.trim().split_whitespace().collect();

    // Parses the first parameter of the users input
    let letter: char = match parameters[0].parse() {
        Err(_) => {
            println!("Invalid Input, Try Again");
            return ('X', 0, false);
        }
        Ok(letter) => letter,
    };

    // Parses the second parameter of the users input
    let col: usize = match parameters[1].parse() {
        Err(_) => {
            println!("Invalid Input, Try Again");
            return ('X', 0, false);
        }
        Ok(col) => col,
    };

    (letter, col, true)
}

fn connect4_cli(lvl : &str) {
    let mut depth;
    match lvl {
        "1" => {
            depth = 0;
        },
        "2" => {
            depth = 1;
        },
        "3" => {
            depth = 2;
        },
        "4" => {
            depth = 3;
        },
        _ => {
            println!("There is an invalid game setting, thus defaulting to human game");
            depth = 0;
        }
    }

    // Initializes the game
    let mut connect4 = Connect4::new();
    let mut active_player = PieceColor::YELLOW;

    // Displays the empty board before the start of the game
    println!("{}", connect4);

    loop {
        let winner = match connect4.check_for_win(active_player) {
            Some(_) => Some(active_player),
            None => None,
        };        // Switch Players

        if winner.is_some() {
            break;
        }

        active_player = active_player.switch();

        // Display who's turn it is, prompt for a column input
        println!("It is the {} player's turn", active_player);
        println!("==========================");
        println!("Enter the column you want to drop your piece in (0-6)");

        if (active_player == PieceColor::RED) || (depth == 0) {
            let mut column: usize;
            let mut is_valid = false;
            // Gets input from the user until their input is valid
            while !is_valid {
                let result = get_input_connect4();
                column = result.0;
                is_valid = result.1;
                if !is_valid {
                    continue;
                }
                is_valid = connect4.drop(column);
            }
        } else {
            connect4.drop(cpu_con4::make_move(connect4.clone(), depth));
        }

        // Displays the board after the input
        println!("{}", connect4);
    }

    println!("{} player won!", active_player);
}

fn toot_and_otto_cli(lvl : &str) {
    let mut depth;
    match lvl {
        "1" => {
            depth = 0;
        },
        "2" => {
            depth = 1;
        },
        "3" => {
            depth = 2;
        },
        "4" => {
            depth = 3;
        },
        _ => {
            println!("There is an invalid game setting, thus defaulting to human game");
            depth = 0;
        }
    }

    // Initializes the game
    let mut toot_and_otto = TootAndOtto::new();

    let mut active_player = Player::TOOT;

    // Displays the empty board before the start of the game
    println!("{}", toot_and_otto);

    loop {
        // Display who's turn it is, prompt for a column input
        println!("It is {}'s turn", active_player);
        println!("==========================");
        println!(
            "Enter the letter you want to drop (O, T) and the column you want to drop it in (0-5)"
        );

        let mut column: usize;
        let mut letter: char;
        let mut is_valid = false;

        // Gets input from the user until their input is valid
        if (depth == 0) || (active_player == Player::TOOT) {
            while !is_valid {
                let result = get_input_toot_and_otto();
                letter = result.0;
                column = result.1;
                is_valid = result.2;

                if !is_valid {
                    continue;
                }

                let drop_piece = match letter {
                    'T' => PieceLetter::T,
                    'O' => PieceLetter::O,
                    _ => {
                        println!("Invalid Input, try again");
                        continue;
                    }
                };

                is_valid = toot_and_otto.drop(drop_piece, column);

                if !is_valid {
                    println!("Your input wasn't valid !");
                }

            }
        } else {
            let res = cpu_toot::make_move(toot_and_otto.clone(), depth);
            toot_and_otto.drop(res.1, res.0);
        }

        // Displays the board after the input
        println!("{}", toot_and_otto);

        // Switch Players
        active_player = active_player.switch();

        // Checks if either player won
        // Unlike connect for, either player could win on any given move
        // Both players could also win of the piece dropped forms "TOOT"
        // and "OTTO" simultaneously
        if toot_and_otto.is_terminal {
            break;
        }
    }

    match toot_and_otto.winner {
        Some(player) => {
            println!("Player {} won!", player);
        }
        None => {
            println!("Game drawn :)");
        }
    }
}

fn main() {
    // Asks the user which game they'd like to play
    println!("Which game would you like to play?");
    println!("1. Connect 4");
    println!("2. TOOT and OTTO");

    let mut game = String::new();
    if let Err(_) = io::stdin().read_line(&mut game) {
        println!("Input failed, try again");
        return;
    };

    println!("----- ===== ----- ===== ----- ===== -----");
    println!("1. Human");
    println!("2. CPU EASY");
    println!("3. CPU MED");
    println!("4. CPU HARD");
    let mut lvl = String::new();
    if let Err(_) = io::stdin().read_line(&mut lvl) {
        println!("Input failed, try again");
        return;
    };

    let game = game.trim();
    if game == "1" {
        connect4_cli(lvl.trim());
    } else {
        toot_and_otto_cli(lvl.trim());
    }
}
