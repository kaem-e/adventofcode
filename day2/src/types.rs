// this might be a bit overkill

use std::char::ParseCharError;

pub enum PlayerChoice {
	Rock,
	Paper,
	Sissors,
}
impl TryFrom<char> for PlayerChoice {
	type Error = color_eyre::Report;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		match c {
			'A' | 'X' => Ok(PlayerChoice::Rock),
			'B' | 'Y' => Ok(PlayerChoice::Paper),
			'C' | 'Z' => Ok(PlayerChoice::Sissors),
			_ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
		}
	}
}

impl PlayerChoice {
	pub fn find_score(&self, opponent: &PlayerChoice) -> u32 {
		match (self, opponent) {
			(PlayerChoice::Rock, PlayerChoice::Rock) => 3 + 1,
			(PlayerChoice::Rock, PlayerChoice::Paper) => 0 + 1,
			(PlayerChoice::Rock, PlayerChoice::Sissors) => 6 + 1,
			(PlayerChoice::Paper, PlayerChoice::Rock) => 6 + 2,
			(PlayerChoice::Paper, PlayerChoice::Paper) => 3 + 2,
			(PlayerChoice::Paper, PlayerChoice::Sissors) => 0 + 2,
			(PlayerChoice::Sissors, PlayerChoice::Rock) => 0 + 3,
			(PlayerChoice::Sissors, PlayerChoice::Paper) => 6 + 3,
			(PlayerChoice::Sissors, PlayerChoice::Sissors) => 3 + 3,
		}
	}
}

pub enum NeededResult {
	Lose,
	Draw,
	Win,
}

impl NeededResult {
	pub fn find_score(&self, opponent: &PlayerChoice) -> u32 {
		match (self, opponent) {
			(NeededResult::Lose, PlayerChoice::Rock) => 3 + 0,
			(NeededResult::Lose, PlayerChoice::Paper) => 1 + 0,
			(NeededResult::Lose, PlayerChoice::Sissors) => 2 + 0,
			(NeededResult::Draw, PlayerChoice::Rock) => 1 + 3,
			(NeededResult::Draw, PlayerChoice::Paper) => 2 + 3,
			(NeededResult::Draw, PlayerChoice::Sissors) => 3 + 3,
			(NeededResult::Win, PlayerChoice::Rock) => 2 + 6,
			(NeededResult::Win, PlayerChoice::Paper) => 3 + 6,
			(NeededResult::Win, PlayerChoice::Sissors) => 1 + 6,
		}
	}
}

mod tests {
	use super::*;

	#[test]
	#[allow(non_snake_case)]
	fn test_NeededResult_scores() {
		assert_eq!(NeededResult::Lose.find_score(&PlayerChoice::Rock), 3);
		assert_eq!(NeededResult::Lose.find_score(&PlayerChoice::Paper), 1);
		assert_eq!(NeededResult::Lose.find_score(&PlayerChoice::Sissors), 2);
		assert_eq!(NeededResult::Draw.find_score(&PlayerChoice::Rock), 4);
		assert_eq!(NeededResult::Draw.find_score(&PlayerChoice::Paper), 5);
		assert_eq!(NeededResult::Draw.find_score(&PlayerChoice::Sissors), 6);
		assert_eq!(NeededResult::Win.find_score(&PlayerChoice::Rock), 8);
		assert_eq!(NeededResult::Win.find_score(&PlayerChoice::Paper), 9);
		assert_eq!(NeededResult::Win.find_score(&PlayerChoice::Sissors), 7);
	}

	#[test]
	fn test_rps_scores() {
		assert_eq!(PlayerChoice::Rock.find_score(&PlayerChoice::Rock), 4);
		assert_eq!(PlayerChoice::Rock.find_score(&PlayerChoice::Paper), 1);
		assert_eq!(PlayerChoice::Rock.find_score(&PlayerChoice::Sissors), 7);
		assert_eq!(PlayerChoice::Paper.find_score(&PlayerChoice::Rock), 8);
		assert_eq!(PlayerChoice::Paper.find_score(&PlayerChoice::Paper), 5);
		assert_eq!(PlayerChoice::Paper.find_score(&PlayerChoice::Sissors), 2);
		assert_eq!(PlayerChoice::Sissors.find_score(&PlayerChoice::Rock), 3);
		assert_eq!(PlayerChoice::Sissors.find_score(&PlayerChoice::Paper), 9);
		assert_eq!(PlayerChoice::Sissors.find_score(&PlayerChoice::Sissors), 6);
	}
}
