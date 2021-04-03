use std::fmt::{Display, Formatter, Result};

/// A wrapper around the primary `Board` component
pub struct Connect4 {
	board: Board,
}

/// The two colors that can be placed on a Connect 4 board
#[derive(Copy, Clone)]
pub enum PieceColor {
	RED,
	YELLOW,
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
		}
	}

	/// Drops a piece of some `color` into a `col` on the the board
	pub fn drop(&mut self, color: PieceColor, col: usize) -> bool {
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
		self.board[row][col] = Some(color);
		true
	}

	/// Gets the number of chips that have been placed in a column
	pub fn get_col_height(&self, col: usize) -> usize {
		for row in 0..NUM_ROWS {
			match self.board[row][col] {
				None => {}
				_ => return NUM_ROWS - row,
			}
		}

		// No chips have been placed in the column
		0
	}

	/// Checks to see if a color has one the game
	pub fn check_for_win(&self, color: PieceColor) -> bool {
		// Searches for four in a row along some linear traversal
		let check_line = |start_y: usize,
		                  start_x: usize,
		                  update: Box<dyn Fn(usize, usize) -> (usize, usize)>|
		 -> bool {
			let mut col = start_x;
			let mut row = start_y;

			let mut consecutive_count = 0;
			while row < NUM_ROWS && col < NUM_COLS {
				match self.board[row][col] {
					None => consecutive_count = 0,
					Some(cell_color) => {
						if color == cell_color {
							consecutive_count += 1;
						} else {
							consecutive_count = 0;
						}
					}
				}

				if consecutive_count == 4 {
					return true;
				}

				let update_coordinates = update(col, row);

				// This handles an edge case in the down left update where
				// col would go from 0 to -1 to indicate it goes out of range
				//
				// This would panic, so instead I keep the value as 0 and then
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
		let dr_starts = [[2, 0], [1, 0], [0, 0], [0, 1], [0, 2], [0, 3]];
		for point in dr_starts.iter() {
			if check_line(point[0], point[1], Box::new(dr_update)) {
				return true;
			}
		}

		// Checks all the down-left diagonals
		let dl_starts = [[1, 6], [1, 6], [0, 6], [0, 5], [0, 4], [0, 3]];
		for point in dl_starts.iter() {
			if check_line(point[0], point[1], Box::new(dl_update)) {
				return true;
			}
		}

		false
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
						PieceColor::RED => print_string.push('R'),
						PieceColor::YELLOW => print_string.push('Y'),
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

impl PartialEq for PieceColor {
	/// Equivalence Checker for the PieceColors
	fn eq(&self, other: &PieceColor) -> bool {
		match self {
			PieceColor::RED => match other {
				PieceColor::RED => true,
				PieceColor::YELLOW => false,
			},
			PieceColor::YELLOW => match other {
				PieceColor::RED => false,
				PieceColor::YELLOW => true,
			},
		}
	}
}

impl Display for PieceColor {
	/// Prints out the piece color
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			PieceColor::RED => write!(f, "{}", "Red"),
			PieceColor::YELLOW => write!(f, "{}", "Yellow"),
		}
	}
}

impl PieceColor {
	/// Flips the value of the piece
	pub fn switch(&self) -> Self {
		match self {
			PieceColor::RED => PieceColor::YELLOW,
			PieceColor::YELLOW => PieceColor::RED,
		}
	}
}
