use super::{
	piece_letter::{PieceLetter, PieceLetter::*},
	player::Player::*,
	toot_and_otto::TootAndOtto,
};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

pub fn make_move(board: TootAndOtto, depth: usize) -> (usize, PieceLetter) {
	let (col, letter, _) = minmax(board, depth, true);
	return (col, letter);
}

fn minmax(board: TootAndOtto, depth: usize, is_cpu_turn: bool) -> (usize, PieceLetter, i32) {
	if board.is_terminal {
		return match board.winner {
			None => (3, O, 0), // Draw
			Some(player) => match player {
				OTTO => (3, O, i32::MAX - 25 + depth as i32), // Computer won, good
				TOOT => (3, O, i32::MIN + 25 - depth as i32), // Human won, bad
			},
		};
	} else if depth == 0 {
		return (3, O, board.calculate_score(OTTO));
	}

	if is_cpu_turn {
		let mut best_options = vec![(0, T, i32::MIN)];

		for letter in PieceLetter::iter() {
			for col in board.get_columns().iter() {
				let mut copy_board = board.clone();

				if copy_board.drop(letter, *col) == false {
					continue;
				}

				let new_value = minmax(copy_board, depth - 1, false).2;

				if new_value == best_options[0].2 {
					best_options.push((*col, letter, new_value));
				} else if new_value > best_options[0].2 {
					best_options = vec![(*col, letter, new_value)];
				}
			}
		}

		*best_options.choose(&mut rand::thread_rng()).unwrap()
	} else {
		let mut best_options = vec![(0, T, i32::MAX)];

		for letter in PieceLetter::iter() {
			for col in board.get_columns().iter() {
				let mut copy_board = board.clone();
				if copy_board.drop(letter, *col) == false {
					continue;
				}

				let new_value = minmax(copy_board, depth - 1, true).2;

				if new_value == best_options[0].2 {
					best_options.push((*col, letter, new_value));
				} else if new_value < best_options[0].2 {
					best_options = vec![(*col, letter, new_value)];
				}
			}
		}

		*best_options.choose(&mut rand::thread_rng()).unwrap()
	}
}
