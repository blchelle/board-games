use crate::connect4::connect4::{Connect4, NUM_COLS};
use rand::seq::SliceRandom;

/// The easy cpu chooses a random available slot
pub fn make_move(board: &Connect4) -> usize {
	let mut eligible_columns = vec![];

	// Finds which columns in the board are available
	for i in 0..NUM_COLS {
		if board.get_col_height(i) < 6 {
			eligible_columns.push(i);
		}
	}

	// Chooses randomly from the eligible columns
	match eligible_columns.choose(&mut rand::thread_rng()) {
		None => panic!("All columns are full, error"),
		Some(col) => *col,
	}
}
