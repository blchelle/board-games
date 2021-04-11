use std::fmt::{Display, Formatter, Result};

use Player::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Player {
	TOOT,
	OTTO,
}

impl Player {
	/// Switches the player to the other value
	pub fn switch(&self) -> Self {
		match self {
			TOOT => OTTO,
			OTTO => TOOT,
		}
	}
}

impl Display for Player {
	/// Prints out the `PieceLetter`
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			TOOT => write!(f, "{}", "TOOT"),
			OTTO => write!(f, "{}", "OTTO"),
		}
	}
}
