mod connect4 {
    pub mod connect4;
    pub mod easy_cpu;
    pub mod hard_cpu;
}
mod toot_and_otto {
    pub mod toot_and_otto;
}

use connect4::{
    connect4::{Connect4, PieceColor},
    easy_cpu, hard_cpu,
};
use std::io;
use toot_and_otto::toot_and_otto::{PieceLetter, Player, TootAndOtto};

/// Gets input from the user and returns a usize and a tuple
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

fn connect4_cli() {
    // Initializes the game
    let mut connect4 = Connect4::new();
    let mut active_player = PieceColor::YELLOW;

    // Displays the empty board before the start of the game
    println!("{}", connect4);

    while !connect4.check_for_win(active_player) {
        // Switch Players
        active_player = active_player.switch();

        // Display who's turn it is, prompt for a column input
        println!("It is the {} player's turn", active_player);
        println!("==========================");
        println!("Enter the column you want to drop your piece in (0-6)");

        if active_player == PieceColor::RED {
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
                is_valid = connect4.drop(active_player, column);
            }
        } else {
            connect4.drop(active_player, hard_cpu::make_move(connect4.clone()));
        }
        // Displays the board after the input
        println!("{}", connect4);
    }

    println!("{} player won!", active_player);
}

fn toot_and_otto_cli() {
    // Initializes the game
    let mut toot_and_otto = TootAndOtto::new();

    let mut active_player = Player::One;

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
        }

        // Displays the board after the input
        println!("{}", toot_and_otto);

        // Switch Players
        active_player = active_player.switch();

        // Checks if either player won
        // Unlike connect for, either player could win on any given move
        // Both players could also win of the piece dropped forms "TOOT"
        // and "OTTO" simultaneously
        let p1_won = toot_and_otto.check_for_win(Player::One);
        let p2_won = toot_and_otto.check_for_win(Player::Two);

        if p1_won && !p2_won {
            println!("Player 1 Won!!!");
            return;
        } else if !p1_won && p2_won {
            println!("Player 2 Won!!!");
            return;
        } else if p1_won && p2_won {
            println!("It's a draw!");
            return;
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

    let game = game.trim();
    if game == "1" {
        connect4_cli();
    } else {
        toot_and_otto_cli();
    }
}
