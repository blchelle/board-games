use super::{connect4::Connect4, piece_color::PieceColor::*};
use std::cmp::{max, min};

pub fn make_move(board: Connect4, depth: usize) -> usize {
	let (col, _) = minmax(board, depth, i32::MIN, i32::MAX, true);
	return col;
}

fn minmax(
	board: Connect4,
	depth: usize,
	mut alpha: i32,
	mut beta: i32,
	is_cpu_turn: bool,
) -> (usize, i32) {
	if board.is_terminal {
		return match board.winner {
			None => (3, 0), // Draw
			Some(color) => match color {
				YELLOW => (3, (i32::MAX - 43 + depth as i32)), // Computer won, good
				RED => (3, i32::MIN + 43 - depth as i32),      // Human won, bad
			},
		};
	} else if depth == 0 {
		return (3, board.calculate_score(YELLOW));
	}

	let mut best_col = 0;

	if is_cpu_turn {
		let mut value = i32::MIN;

		for col in board.get_columns().iter() {
			let mut copy_board = board.clone();

			// log::info!("==========Column {} for {}==========", col, YELLOW);
			if copy_board.drop(YELLOW, *col) == false {
				continue;
			}
			let new_value = minmax(copy_board, depth - 1, alpha, beta, false).1;
			alpha = max(alpha, new_value);
			if alpha >= beta {
				break;
			}

			if new_value > value {
				value = new_value;
				best_col = *col;
			}
		}

		return (best_col, value);
	} else {
		let mut value = i32::MAX;
		for col in board.get_columns().iter() {
			let mut copy_board = board.clone();
			if copy_board.drop(RED, *col) == false {
				continue;
			}
			// log::info!("==========Column {} for {}==========", col, RED);
			let new_value = minmax(copy_board, depth - 1, alpha, beta, true).1;
			beta = min(beta, new_value);
			if alpha >= beta {
				break;
			}
			// let new_value = minimax_with_ab_pruning(copy_board, depth - 1, true).1;

			if new_value < value {
				value = new_value;
				best_col = *col;
			}
		}

		return (best_col, value);
	}
}
