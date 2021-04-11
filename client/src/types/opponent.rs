use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone)]
pub enum Opponent {
	Human,
	EasyCPU,
	MediumCPU,
	HardCPU,
}

impl Display for Opponent {
	/// Prints out the piece color
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			Opponent::Human => write!(f, "{}", "Human"),
			Opponent::EasyCPU => write!(f, "{}", "Easy CPU"),
			Opponent::MediumCPU => write!(f, "{}", "Medium CPU"),
			Opponent::HardCPU => write!(f, "{}", "Hard CPU"),
		}
	}
}

impl PartialEq for Opponent {
	fn eq(&self, other: &Opponent) -> bool {
		use Opponent::*;

		match (self, other) {
			(Human, Human) => true,
			(EasyCPU, EasyCPU) => true,
			(MediumCPU, MediumCPU) => true,
			(HardCPU, HardCPU) => true,
			_ => false,
		}
	}
}
