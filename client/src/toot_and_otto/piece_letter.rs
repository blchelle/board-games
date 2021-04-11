use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;

use PieceLetter::*;

/// The two pieces that can be placed on a TOOT-n-OTTO board
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum PieceLetter {
	T,
	O,
}

impl Display for PieceLetter {
	/// Prints out the `PieceLetter`
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			O => write!(f, "{}", "O"),
			T => write!(f, "{}", "T"),
		}
	}
}
