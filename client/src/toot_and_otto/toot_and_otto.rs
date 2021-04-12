use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

use super::{
	piece_letter::{PieceLetter, PieceLetter::*},
	player::{Player, Player::*},
};

/// A wrapper around the primary `Board` component
#[derive(Clone, Copy)]
pub struct TootAndOtto {
	pub board: Board,
	pub winner: Option<Player>,
	pub active_player: Player,
	pub is_terminal: bool,
	pub moves_played: usize,
	pub column_heights: [usize; NUM_COLS],
	pub piece_counts: [[usize; 2]; 2], // [[TOOTS T's, TOOTS O's], [OTTO's T's, OTTO's O's]]
}

/// A 6x4 TOOT-n-OTTO Board
type Board = [[BoardCell; NUM_COLS]; NUM_ROWS];
pub const NUM_COLS: usize = 6;
pub const NUM_ROWS: usize = 4;

/// All the possible values for a board cell (None, Some(T), SOME(O))
type BoardCell = Option<PieceLetter>;

impl TootAndOtto {
	/// Initializes an empty TOOT-n-OTTO board
	pub fn new() -> Self {
		TootAndOtto {
			board: [[None; NUM_COLS]; NUM_ROWS],
			column_heights: [0; NUM_COLS],
			active_player: TOOT,
			is_terminal: false,
			moves_played: 0,
			winner: None,
			piece_counts: [[6; 2]; 2],
		}
	}

	/// Drops a piece of some `letter` into a `col` on the the board
	pub fn drop(&mut self, letter: PieceLetter, col: usize) -> bool {
		// Checks for some simple input errors
		if col >= NUM_COLS {
			println!("Invalid Drop Column, Try Again.");
			return false;
		}

		// Gets the number of pieces in the column and checks if it is full
		let col_height = self.get_col_height(col);

		if col_height == NUM_ROWS {
			println!("Column {} is full, Try Again.", col);
			return false;
		}

		// Calculates which row the the piece should be inserted into
		let row = NUM_ROWS - 1 - col_height;

		// Inserts the piece into the board
		self.board[row][col] = Some(letter);

		let toot_won = self.check_for_win(TOOT);
		let otto_won = self.check_for_win(OTTO);

		self.winner = match (toot_won, otto_won) {
			(true, true) => {
				self.is_terminal = true;
				None
			}
			(false, true) => {
				self.is_terminal = true;
				Some(OTTO)
			}
			(true, false) => {
				self.is_terminal = true;
				Some(TOOT)
			}
			(false, false) => None,
		};

		// Updates the piece count for the player
		match self.active_player {
			TOOT => match letter {
				T => self.piece_counts[0][0] -= 1,
				O => self.piece_counts[0][1] -= 1,
			},
			OTTO => match letter {
				T => self.piece_counts[1][0] -= 1,
				O => self.piece_counts[1][1] -= 1,
			},
		}

		self.moves_played += 1;
		if self.moves_played > NUM_COLS * NUM_ROWS {
			self.is_terminal = true
		}

		self.active_player = self.active_player.switch();

		true
	}

	/// Gets the number of chips that have been placed in a column
	fn get_col_height(&self, col: usize) -> usize {
		for row in 0..NUM_ROWS {
			match self.board[row][col] {
				None => {}
				_ => return NUM_ROWS - row,
			}
		}

		// No chips have been placed in the column
		0
	}

	// Checks to see if a color has one the game
	pub fn check_for_win(&self, player: Player) -> bool {
		let win_pattern = match player {
			TOOT => [T, O, O, T],
			OTTO => [O, T, T, O],
		};

		// Searches for four in a row along some linear traversal
		let check_line = |start_y: usize,
		                  start_x: usize,
		                  update: Box<dyn Fn(usize, usize) -> (usize, usize)>|
		 -> bool {
			let mut col = start_x;
			let mut row = start_y;

			let mut pattern_index = 0;

			while row < NUM_ROWS && col < NUM_COLS {
				match self.board[row][col] {
					None => pattern_index = 0,
					Some(letter) => {
						if letter == win_pattern[pattern_index] {
							pattern_index += 1;
						} else if letter == win_pattern[0] {
							pattern_index = 1;
						} else {
							pattern_index = 0;
						}
					}
				}

				if pattern_index == 4 {
					return true;
				}

				let update_coordinates = update(col, row);

				// This handles an edge case in the down left update where
				// col would go from 0 to -1 to indicate it goes out of range
				//
				// This would panics, so instead I keep the value as 0 and then
				// check if the value changes.
				if col != update_coordinates.0 || row != NUM_ROWS {
					col = update_coordinates.0;
					row = update_coordinates.1;
				} else {
					return false;
				}
			}

			false
		};

		// Update closure functions for the four traversals
		let row_update = |x: usize, y: usize| -> (usize, usize) { (x + 1, y) };
		let col_update = |x: usize, y: usize| -> (usize, usize) { (x, y + 1) };
		let dr_update = |x: usize, y: usize| -> (usize, usize) { (x + 1, y + 1) };
		let dl_update =
			|x: usize, y: usize| -> (usize, usize) { (if x == 0 { 0 } else { x - 1 }, y + 1) };

		// Checks all the rows
		for row in 0..NUM_ROWS {
			if check_line(row, 0, Box::new(row_update)) {
				return true;
			}
		}

		// Checks all the columns
		for col in 0..NUM_COLS {
			if check_line(0, col, Box::new(col_update)) {
				return true;
			}
		}

		// Checks all the down-right diagonals
		let dr_starts = [[0, 0], [0, 1], [0, 2]];
		for point in dr_starts.iter() {
			if check_line(point[0], point[1], Box::new(dr_update)) {
				return true;
			}
		}

		// Checks all the down-left diagonals
		let dl_starts = [[0, 5], [0, 4], [0, 3]];
		for point in dl_starts.iter() {
			if check_line(point[0], point[1], Box::new(dl_update)) {
				return true;
			}
		}

		false
	}

	pub fn calculate_score(&self, player: Player) -> i32 {
		const _CENTER_COL: i32 = 5; // Playing the center column
		const _LINE_OF_TWO: i32 = 1; // Two pieces in a line
		const _LINE_OF_THREE: i32 = 10; // 3 pieces in a line
		const _LINE_OF_FOUR: i32 = 100_000; // Self Won

		const _OPPONENT_LINE_OF_THREE: i32 = -20; // Opponent can win
		const _OPPONENT_LINE_OF_THREE_WITH_BELOW: i32 = -10_000; // Opponent can win
		const _OPPONENT_LINE_OF_TWO: i32 = -3; // Opponent can setup for win

		let win_pattern = match player {
			TOOT => [T, O, O, T],
			OTTO => [O, T, T, O],
		};

		let mut score = 0;

		let calculate_window_score = |window: &[(BoardCell, bool)]| -> i32 {
			let mut own_count = 0;
			let mut opponent_count = 0;
			let mut empty_no_below_count = 0;
			let mut empty_with_below_count = 0;

			for (i, cell) in window.iter().enumerate() {
				match cell.0 {
					None => match window[0].1 {
						false => empty_no_below_count += 1,
						true => empty_with_below_count += 1,
					},
					Some(l) => {
						if l == win_pattern[i] {
							own_count += 1
						} else {
							opponent_count += 1
						}
					}
				}
			}

			if own_count > 0 && opponent_count > 0 {
				return 0;
			}

			match (
				own_count,
				opponent_count,
				empty_with_below_count,
				empty_no_below_count,
			) {
				(4, 0, 0, 0) => _LINE_OF_FOUR,
				(3, 0, _, _) => _LINE_OF_THREE,
				(2, 0, _, _) => _LINE_OF_TWO,
				(0, 3, 1, 0) => _OPPONENT_LINE_OF_THREE_WITH_BELOW,
				(0, 3, 0, 1) => _OPPONENT_LINE_OF_THREE,
				(0, 2, _, _) => _OPPONENT_LINE_OF_TWO,
				_ => 0,
			}
		};

		// Performs a check across all rows
		for row in 0..NUM_ROWS {
			for start_col in 0..NUM_COLS - 3 {
				let mut window: Vec<(BoardCell, bool)> = vec![];
				(start_col..start_col + 4).into_iter().for_each(|col| {
					window.push((
						self.board[row][col],
						self.column_heights[col] >= NUM_ROWS - row - 1,
					))
				});
				score += calculate_window_score(&window);
			}
		}

		let h_points = score;
		// log::info!("Horizontal {}: {}", player, h_points);
		// Performs a check across all columns
		for col in 0..NUM_COLS {
			for start_row in 0..NUM_ROWS - 3 {
				let mut window: Vec<(BoardCell, bool)> = vec![];
				(start_row..start_row + 4)
					.into_iter()
					.for_each(|row| window.push((self.board[row][col], true)));
				score += calculate_window_score(&window);
			}
		}
		let v_points = score - h_points;
		// log::info!("Vertical {}: {}", player, v_points);

		// Perform a check across positively sloped diagonals
		for col in 0..NUM_COLS - 3 {
			let mut window: Vec<(BoardCell, bool)> = vec![];
			(0..4).into_iter().for_each(|i| {
				window.push((
					self.board[NUM_ROWS - 1 - i][col + i],
					self.column_heights[col + i] >= i,
				))
			});
			score += calculate_window_score(&window);
		}

		let pd_points = score - h_points - v_points;
		// log::info!("Positive Diagonal {}: {}", player, pd_points);
		// Perform a check across positively sloped diagonals
		for col in 0..NUM_COLS - 3 {
			let mut window = vec![];
			(0..4).into_iter().for_each(|i| {
				window.push((
					self.board[i][col + i],
					self.column_heights[col + i] >= NUM_ROWS - i - 1,
				))
			});
			score += calculate_window_score(&window);
		}

		let nd_points = score - h_points - v_points - pd_points;
		// log::info!("Negative Diagonal {}: {}", player, nd_points);

		score
	}

	pub fn get_columns(&self) -> [usize; NUM_COLS] {
		[2, 3, 1, 4, 0, 5]
	}
}

impl Display for TootAndOtto {
	/**
	 * Defines the print functionality for the board
	 */
	fn fmt(&self, f: &mut Formatter) -> Result {
		// Initializes an empty string that will be built on
		let mut print_string = String::new();

		for row in 0..NUM_ROWS {
			for col in 0..NUM_COLS {
				// Places a -, R, Y depending on what is in the cell
				match self.board[row][col] {
					None => print_string.push('-'),
					Some(color) => print_string.push_str(format!("{}", color).as_str()),
				};

				print_string.push(' ');
			}

			print_string.push('\n');
		}

		print_string.push_str("0 1 2 3 4 5");

		write!(f, "\nCurrent Board:\n{}\n", print_string)
	}
}
