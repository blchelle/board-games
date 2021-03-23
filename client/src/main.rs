mod connect4;

use connect4::{Connect4, PieceColor};

fn main() {
    let mut connect4 = Connect4::new();

    connect4.drop(PieceColor::RED, 0);
    println!("{}", connect4);
    println!("{}", connect4.check_for_win(PieceColor::RED));

    connect4.drop(PieceColor::RED, 0);
    println!("{}", connect4);
    println!("{}", connect4.check_for_win(PieceColor::RED));

    connect4.drop(PieceColor::RED, 0);
    println!("{}", connect4);
    println!("{}", connect4.check_for_win(PieceColor::RED));

    connect4.drop(PieceColor::RED, 1);
    println!("{}", connect4);
    println!("{}", connect4.check_for_win(PieceColor::RED));
}
