use super::{
	connect4::{Connect4, NUM_COLS, NUM_ROWS},
	piece_color::{PieceColor, PieceColor::*},
};

pub fn make_move(board: Connect4) -> usize {
	return negamax(board, YELLOW).1;
}

pub fn negamax(board: Connect4, active_player: PieceColor) -> (i32, usize) {
	if board.moves_played == NUM_COLS * NUM_ROWS {
		return (0, 0);
	}

	if let Some(col) = check_for_winning_move(active_player, board.clone()) {
		return (
			((NUM_COLS * NUM_ROWS) + 1 - (board.moves_played / 2)) as i32,
			col,
		);
	}

	let mut best_score = -1 * ((NUM_COLS * NUM_ROWS) as i32);
	let mut best_column = 0;

	for col in 0..NUM_COLS {
		if board.get_col_height(col) == NUM_ROWS {
			continue;
		}

		let mut next_board = board.clone();
		next_board.drop(active_player, col);

		let (score, _) = negamax(next_board, active_player.switch());
		if score > best_score {
			best_score = score;
			best_column = col
		}
	}

	(best_score, best_column)
}

/// Checks if there is a move that will lead to a win
fn check_for_winning_move(color: PieceColor, board: Connect4) -> Option<usize> {
	for i in 0..NUM_COLS {
		let mut temp_board = board.clone();
		temp_board.drop(color, i);

		if temp_board.check_for_win(color) {
			return Some(i);
		}
	}

	None
}
