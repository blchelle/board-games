use std::fmt::{Display, Formatter, Result};

/// A wrapper around the primary `Board` component
pub struct TootAndOtto {
	board: Board,
}

/// The two colors that can be placed on a Connect 4 board
#[derive(Copy, Clone)]
pub enum PieceLetter {
	T,
	O,
}

#[derive(Copy, Clone)]
pub enum Player {
	One,
	Two,
}

/// A 7x6 Connect 4 Board
type Board = [[BoardCell; NUM_COLS]; NUM_ROWS];
const NUM_COLS: usize = 6;
const NUM_ROWS: usize = 4;

/// All the possible values for a board cell (None, Some(T), SOME(O))
type BoardCell = Option<PieceLetter>;

impl TootAndOtto {
	/// Initializes an empty Connect 4 board
	pub fn new() -> Self {
		TootAndOtto {
			board: [[None; NUM_COLS]; NUM_ROWS],
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
			Player::One => [
				PieceLetter::T,
				PieceLetter::O,
				PieceLetter::O,
				PieceLetter::T,
			],
			Player::Two => [
				PieceLetter::O,
				PieceLetter::T,
				PieceLetter::T,
				PieceLetter::O,
			],
		};

		// Checks a row to see if there is 4 in a row
		let check_row = |row: usize| -> bool {
			let mut pattern_index = 0;

			for col in 0..NUM_COLS {
				match self.board[row][col] {
					None => pattern_index = 0,
					Some(letter) => {
						if letter == win_pattern[pattern_index] {
							pattern_index += 1;
						} else if letter == win_pattern[0] {
							pattern_index = 1;
						} else {
							pattern_index = 0
						}
					}
				}

				if pattern_index == 4 {
					return true;
				}
			}

			false
		};

		// Checks a column to see if there is 4 in a row
		let check_col = |col: usize| -> bool {
			let mut pattern_index = 0;

			for row in 0..NUM_ROWS {
				match self.board[row][col] {
					None => pattern_index = 0,
					Some(letter) => {
						if letter == win_pattern[pattern_index] {
							pattern_index += 1;
						} else if letter == win_pattern[0] {
							pattern_index = 1;
						} else {
							pattern_index = 0
						}
					}
				}

				if pattern_index == 4 {
					return true;
				}
			}

			false
		};

		// TODO: Check if there is a win along the diagonals

		// Checks all the rows
		for row in 0..NUM_ROWS {
			if check_row(row) {
				return true;
			}
		}

		// Checks all the columns
		for col in 0..NUM_COLS {
			if check_col(col) {
				return true;
			}
		}

		false
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

impl PartialEq for PieceLetter {
	/// Equivalence Checker for the `PieceLetters`
	fn eq(&self, other: &PieceLetter) -> bool {
		match self {
			PieceLetter::T => match other {
				PieceLetter::T => true,
				PieceLetter::O => false,
			},
			PieceLetter::O => match other {
				PieceLetter::T => false,
				PieceLetter::O => true,
			},
		}
	}
}

impl Display for PieceLetter {
	/// Prints out the `PieceLetter`
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			PieceLetter::O => write!(f, "{}", "O"),
			PieceLetter::T => write!(f, "{}", "T"),
		}
	}
}

impl Player {
	/// Switches the player to the other value
	pub fn switch(&self) -> Self {
		match self {
			Player::One => Player::Two,
			Player::Two => Player::One,
		}
	}
}

impl Display for Player {
	/// Prints out the `PieceLetter`
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			Player::One => write!(f, "{}", "Player 1"),
			Player::Two => write!(f, "{}", "Player 2"),
		}
	}
}
