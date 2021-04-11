use super::{
	connect4::{Connect4, NUM_COLS, NUM_ROWS},
	piece_color::{PieceColor, PieceColor::*},
};

/// The hard cpu is not perfect but follows a set of logic
pub fn make_move(board: Connect4) -> usize {
	// The first priority is to check for a winning move
	if let Some(col) = check_for_winning_move(YELLOW, board) {
		return col;
	}

	// The second priority is to stop any winning moves from the opponent
	if let Some(col) = check_for_winning_move(RED, board) {
		return col;
	}

	// The third priority is to actively avoid any move that will set the
	// opponent up on the subsequent turn
	let mut eligible_columns = check_for_eligible_columns(board);
	let bad_columns = check_for_setup_moves(board);

	eligible_columns.sort_by(|a, b| {
		// Set the priority based on distance from the middle
		let mut true_a = (3 - (*a as i32)).abs();
		let mut true_b = (3 - (*b as i32)).abs();

		// If either is in bad_columns, give it a value of inf
		if bad_columns.contains(a) {
			true_a = i32::MAX;
		}

		if bad_columns.contains(b) {
			true_b = i32::MAX;
		}

		true_a.cmp(&true_b)
	});

	// Priority four, prevent "the double threat"

	eligible_columns[0]
}

/// Checks if there is a move that will lead to a win
fn check_for_winning_move(color: PieceColor, board: Connect4) -> Option<usize> {
	for i in 0..NUM_COLS {
		let mut temp_board = board.clone();
		temp_board.drop(color, i);

		if temp_board.check_for_win(color) {
			// println!("Found win for {} on column {}", color, i);
			return Some(i);
		}
	}

	None
}

/// Creates a list of moves that would set the opponent up to win
/// These moves should not be pursued unless they are the only options
fn check_for_setup_moves(board: Connect4) -> Vec<usize> {
	let mut bad_moves = vec![];

	for i in 0..NUM_COLS {
		let mut temp_board = board.clone();

		// Simulates the next two moves if both players play in the same column
		temp_board.drop(YELLOW, i);
		temp_board.drop(RED, i);

		if temp_board.check_for_win(RED) {
			// println!("Found setup for {} on column {}", RED, i);
			bad_moves.push(i);
		}
	}

	bad_moves
}

fn check_for_eligible_columns(board: Connect4) -> Vec<usize> {
	let mut eligible_columns = vec![];

	// Finds which columns in the board are available
	for i in 0..NUM_COLS {
		if board.get_col_height(i) < 6 {
			eligible_columns.push(i);
		}
	}

	eligible_columns
}
