// this might be a bit overkill

#[allow(non_camel_case_types)]
pub enum rps {
	Rock,
	Paper,
	Sissors,
}
impl rps {
	pub fn find_score(&self, opponent: &rps) -> u32 {
		match (self, opponent) {
			(rps::Rock, rps::Rock) => 3 + 1,
			(rps::Rock, rps::Paper) => 0 + 1,
			(rps::Rock, rps::Sissors) => 6 + 1,
			(rps::Paper, rps::Rock) => 6 + 2,
			(rps::Paper, rps::Paper) => 3 + 2,
			(rps::Paper, rps::Sissors) => 0 + 2,
			(rps::Sissors, rps::Rock) => 0 + 3,
			(rps::Sissors, rps::Paper) => 6 + 3,
			(rps::Sissors, rps::Sissors) => 3 + 3,
		}
	}
}

pub enum NeededResult {
	Lose,
	Draw,
	Win,
}

impl NeededResult {
	pub fn find_score(&self, opponent: &rps) -> u32 {
		match (self, opponent) {
			(NeededResult::Lose, rps::Rock) => 3 + 0,
			(NeededResult::Lose, rps::Paper) => 1 + 0,
			(NeededResult::Lose, rps::Sissors) => 2 + 0,
			(NeededResult::Draw, rps::Rock) => 1 + 3,
			(NeededResult::Draw, rps::Paper) => 2 + 3,
			(NeededResult::Draw, rps::Sissors) => 3 + 3,
			(NeededResult::Win, rps::Rock) => 2 + 6,
			(NeededResult::Win, rps::Paper) => 3 + 6,
			(NeededResult::Win, rps::Sissors) => 1 + 6,
		}
	}
}

mod tests {
	use super::*;

	#[test]
	#[allow(non_snake_case)]
	fn test_NeededResult_scores() {
		assert_eq!(NeededResult::Lose.find_score(&rps::Rock), 3);
		assert_eq!(NeededResult::Lose.find_score(&rps::Paper), 1);
		assert_eq!(NeededResult::Lose.find_score(&rps::Sissors), 2);
		assert_eq!(NeededResult::Draw.find_score(&rps::Rock), 4);
		assert_eq!(NeededResult::Draw.find_score(&rps::Paper), 5);
		assert_eq!(NeededResult::Draw.find_score(&rps::Sissors), 6);
		assert_eq!(NeededResult::Win.find_score(&rps::Rock), 8);
		assert_eq!(NeededResult::Win.find_score(&rps::Paper), 9);
		assert_eq!(NeededResult::Win.find_score(&rps::Sissors), 7);
	}

	#[test]
	fn test_rps_scores() {
		assert_eq!(rps::Rock.find_score(&rps::Rock), 4);
		assert_eq!(rps::Rock.find_score(&rps::Paper), 1);
		assert_eq!(rps::Rock.find_score(&rps::Sissors), 7);
		assert_eq!(rps::Paper.find_score(&rps::Rock), 8);
		assert_eq!(rps::Paper.find_score(&rps::Paper), 5);
		assert_eq!(rps::Paper.find_score(&rps::Sissors), 2);
		assert_eq!(rps::Sissors.find_score(&rps::Rock), 3);
		assert_eq!(rps::Sissors.find_score(&rps::Paper), 9);
		assert_eq!(rps::Sissors.find_score(&rps::Sissors), 6);
	}
}
