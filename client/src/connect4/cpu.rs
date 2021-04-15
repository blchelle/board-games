use super::{connect4::Connect4, piece_color::PieceColor::*};
use rand::seq::SliceRandom; // 0.7.2

pub fn make_move(board: Connect4, depth: usize) -> usize {
	let (col, _) = minmax(board, depth, true);
	col
}

fn minmax(board: Connect4, depth: usize, is_cpu_turn: bool) -> (usize, i32) {
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

	let mut best_col: usize = 0;

	if is_cpu_turn {
		let mut value = i32::MIN;

		for col in board.get_columns().iter() {
			let mut copy_board = board.clone();

			if copy_board.drop(*col) == false {
				continue;
			}
			let new_value = minmax(copy_board, depth - 1, false).1;

			if new_value == value {
				let random_choice = *vec![(new_value, *col), (value, best_col)]
					.choose(&mut rand::thread_rng())
					.unwrap();

				value = random_choice.0;
				best_col = random_choice.1;
			} else if new_value > value {
				value = new_value;
				best_col = *col;
			}
		}

		return (best_col, value);
	} else {
		let mut value = i32::MAX;
		for col in board.get_columns().iter() {
			let mut copy_board = board.clone();
			if copy_board.drop(*col) == false {
				continue;
			}
			// log::info!("==========Column {} for {}==========", col, RED);
			let new_value = minmax(copy_board, depth - 1, true).1;
			// let new_value = minimax_with_ab_pruning(copy_board, depth - 1, true).1;

			if new_value == value {
				let random_choice = *vec![(new_value, *col), (value, best_col)]
					.choose(&mut rand::thread_rng())
					.unwrap();

				value = random_choice.0;
				best_col = random_choice.1;
			} else if new_value < value {
				value = new_value;
				best_col = *col;
			}
		}

		return (best_col, value);
	}
}
