use std::fmt::{Display, Formatter, Result};

/// The two colors that can be placed on a Connect 4 board
#[derive(Copy, Clone, PartialEq)]
pub enum PieceColor {
	RED,
	YELLOW,
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

impl Display for PieceColor {
	/// Prints out the piece color
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			PieceColor::RED => write!(f, "{}", "Red"),
			PieceColor::YELLOW => write!(f, "{}", "Yellow"),
		}
	}
}
