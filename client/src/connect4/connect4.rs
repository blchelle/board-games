use super::piece_color::{PieceColor, PieceColor::*};
use std::fmt::{Display, Formatter, Result};

/// A wrapper around the primary `Board` component
#[derive(Copy, Clone)]
pub struct Connect4 {
	pub board: Board,
	pub active_player: PieceColor,
	pub moves_played: usize,
	pub is_terminal: bool,
	pub winner: Option<PieceColor>,
	pub column_heights: [usize; NUM_COLS],
}

/// A 7x6 Connect 4 Board
type Board = [[BoardCell; NUM_COLS]; NUM_ROWS];
pub const NUM_COLS: usize = 7;
pub const NUM_ROWS: usize = 6;

/// All the possible values for a board cell (None, Some(RED), SOME(YELLOW))
type BoardCell = Option<PieceColor>;

impl Connect4 {
	/// Initializes an empty Connect 4 board
	pub fn new() -> Self {
		Connect4 {
			board: [[None; NUM_COLS]; NUM_ROWS],
			moves_played: 0,
			active_player: RED,
			is_terminal: false,
			winner: None,
			column_heights: [0; NUM_COLS],
		}
	}

	/// Drops a piece of some `color` into a `col` on the the board
	pub fn drop(&mut self, col: usize) -> bool {
		// Checks for some simple input errors
		if col >= NUM_COLS {
			return false;
		}

		// Gets the number of pieces in the column and checks if it is full
		let col_height = self.column_heights[col];

		// Checks if the column can be dropped into
		if col_height == NUM_ROWS {
			return false;
		}

		// Calculates which row the the piece should be inserted into
		let row = NUM_ROWS - 1 - col_height;

		// Inserts the piece into the board
		self.board[row][col] = Some(self.active_player);
		self.moves_played += 1;
		self.winner = match self.check_for_win(self.active_player) {
			Some(_) => Some(self.active_player),
			None => None,
		};
		self.is_terminal = self.winner != None || self.moves_played == NUM_COLS * NUM_ROWS;
		self.column_heights[col] += 1;
		self.active_player = self.active_player.switch();

		true
	}

	/// Checks to see if a color has one the game
	pub fn check_for_win(&self, color: PieceColor) -> Option<Vec<[usize; 2]>> {
		let check_for_win_in_window = |window: &[BoardCell]| -> bool {
			for cell in window.iter() {
				match cell {
					None => return false,
					Some(c) => {
						if *c != color {
							return false;
						}
					}
				}
			}

			true
		};

		// Checks all the rows
		for row in 0..NUM_ROWS {
			for start_col in 0..NUM_COLS - 3 {
				let window = &self.board[row][start_col..start_col + 4];
				if check_for_win_in_window(window) {
					return Some((0..4).into_iter().map(|i| [row, start_col + i]).collect());
				}
			}
		}

		// Performs a check across all columns
		for col in 0..NUM_COLS {
			for start_row in 0..NUM_ROWS - 3 {
				let mut window = vec![];
				(start_row..start_row + 4)
					.into_iter()
					.for_each(|row| window.push(self.board[row][col]));

				if check_for_win_in_window(&window) {
					return Some((0..4).into_iter().map(|i| [start_row + i, col]).collect());
				}
			}
		}

		// Perform a check across positively sloped diagonals
		for row in NUM_ROWS - 3..NUM_ROWS {
			for col in 0..NUM_COLS - 3 {
				let mut window = vec![];
				(0..4)
					.into_iter()
					.for_each(|i| window.push(self.board[row - i][col + i]));
				if check_for_win_in_window(&window) {
					return Some((0..4).into_iter().map(|i| [row - i, col + i]).collect());
				}
			}
		}

		// Perform a check across positively sloped diagonals
		for row in 0..NUM_ROWS - 3 {
			for col in 0..NUM_COLS - 3 {
				let mut window = vec![];
				(0..4)
					.into_iter()
					.for_each(|i| window.push(self.board[row + i][col + i]));

				if check_for_win_in_window(&window) {
					return Some((0..4).into_iter().map(|i| [row + i, col + i]).collect());
				}
			}
		}

		None
	}

	/// Calculates a heuristic score for the current player and board position
	pub fn calculate_score(&self, color: PieceColor) -> i32 {
		const _CENTER_COL: i32 = 5; // Playing the center column
		const _LINE_OF_TWO: i32 = 1; // Two pieces in a line
		const _LINE_OF_THREE: i32 = 10; // 3 pieces in a line
		const _LINE_OF_FOUR: i32 = 100_000; // Self Won

		const _OPPONENT_LINE_OF_THREE: i32 = -20; // Opponent can win
		const _OPPONENT_LINE_OF_THREE_WITH_BELOW: i32 = -10_000; // Opponent can win
		const _OPPONENT_LINE_OF_TWO: i32 = -3; // Opponent can setup for win

		let mut score = 0;

		let calculate_window_score = |window: &[(BoardCell, bool)]| -> i32 {
			let mut own_count = 0;
			let mut opponent_count = 0;
			let mut empty_no_below_count = 0;
			let mut empty_with_below_count = 0;

			for cell in window.iter() {
				match cell.0 {
					None => match cell.1 {
						false => empty_no_below_count += 1,
						true => empty_with_below_count += 1,
					},
					Some(c) => {
						if c == color {
							own_count += 1;
						} else {
							opponent_count += 1;
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

		// Perform a check across positively sloped diagonals
		for row in NUM_ROWS - 3..NUM_ROWS {
			for col in 0..NUM_COLS - 3 {
				let mut window: Vec<(BoardCell, bool)> = vec![];
				(0..4).into_iter().for_each(|i| {
					window.push((
						self.board[row - i][col + i],
						self.column_heights[col + i] >= NUM_ROWS - (row - i) - 1,
					))
				});
				score += calculate_window_score(&window);
			}
		}

		// Perform a check across positively sloped diagonals
		for row in 0..NUM_ROWS - 3 {
			for col in 0..NUM_COLS - 3 {
				let mut window = vec![];
				(0..4).into_iter().for_each(|i| {
					window.push((
						self.board[row + i][col + i],
						self.column_heights[col + i] >= NUM_ROWS - (row + i) - 1,
					))
				});
				score += calculate_window_score(&window);
			}
		}

		// Gives +2 points for every block in the center column
		for row in 0..NUM_ROWS {
			match self.board[row][3] {
				None => {}
				Some(c) => {
					if c == color {
						score += _CENTER_COL
					}
				}
			}
		}

		score
	}

	/// Gets the columns ordered by distance from the center
	pub fn get_columns(&self) -> [usize; NUM_COLS] {
		return [3, 2, 4, 1, 5, 0, 6];
	}
}

impl Display for Connect4 {
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
					Some(color) => match color {
						RED => print_string.push('\u{25CF}'),
						YELLOW => print_string.push('\u{25CB}'),
					},
				};

				print_string.push(' ');
			}

			print_string.push('\n');
		}

		print_string.push_str("0 1 2 3 4 5 6");

		write!(f, "\nCurrent Board:\n{}\n", print_string)
	}
}
