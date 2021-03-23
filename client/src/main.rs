mod connect4;

use connect4::{Connect4, PieceColor};
use std::io;

/// Gets input from the user and returns a usize and a tuple
fn get_input() -> (usize, bool) {
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

fn main() {
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

        let mut column: usize;
        let mut is_valid = false;

        // Gets input from the user until their input is valid
        while !is_valid {
            let result = get_input();
            column = result.0;
            is_valid = result.1;

            if !is_valid {
                continue;
            }

            is_valid = connect4.drop(active_player, column);
        }

        // Displays the board after the input
        println!("{}", connect4);
    }

    println!("{} player won!", active_player);
}
