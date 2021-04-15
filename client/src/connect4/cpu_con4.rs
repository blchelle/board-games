use super::{connect4::Connect4, piece_color::PieceColor::*};
use rand::seq::SliceRandom;

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

	if is_cpu_turn {
		let mut best_options = vec![(0, i32::MIN)];

		for col in board.get_columns().iter() {
			let mut copy_board = board.clone();

			if copy_board.drop(*col) == false {
				continue;
			}
			let new_value = minmax(copy_board, depth - 1, false).1;

			if new_value == best_options[0].1 {
				best_options.push((*col, new_value));
			} else if new_value > best_options[0].1 {
				best_options = vec![(*col, new_value)];
			}
		}

		*best_options.choose(&mut rand::thread_rng()).unwrap()
	} else {
		let mut best_options = vec![(0, i32::MAX)];

		for col in board.get_columns().iter() {
			let mut copy_board = board.clone();
			if copy_board.drop(*col) == false {
				continue;
			}

			let new_value = minmax(copy_board, depth - 1, true).1;

			if new_value == best_options[0].1 {
				best_options.push((*col, new_value));
			} else if new_value < best_options[0].1 {
				best_options = vec![(*col, new_value)];
			}
		}

		*best_options.choose(&mut rand::thread_rng()).unwrap()
	}
}
