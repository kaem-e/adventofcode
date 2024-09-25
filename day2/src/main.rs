#![allow(unused)]

mod input;

// this might be a bit overkill
enum RPS {
	Rock,
	Paper,
	Sissors,
}
impl RPS {
	fn find_score(&self, opponent: &RPS) -> u32 {
		match (self, opponent) {
			(RPS::Rock, RPS::Rock) => 3 + 1,
			(RPS::Rock, RPS::Paper) => 0 + 1,
			(RPS::Rock, RPS::Sissors) => 6 + 1,
			(RPS::Paper, RPS::Rock) => 6 + 2,
			(RPS::Paper, RPS::Paper) => 3 + 2,
			(RPS::Paper, RPS::Sissors) => 0 + 2,
			(RPS::Sissors, RPS::Rock) => 0 + 3,
			(RPS::Sissors, RPS::Paper) => 6 + 3,
			(RPS::Sissors, RPS::Sissors) => 3 + 3,
		}
	}
}

#[test]
fn test_rps_scores() {
	assert_eq!(RPS::Rock.find_score(&RPS::Rock), 4);
	assert_eq!(RPS::Rock.find_score(&RPS::Paper), 1);
	assert_eq!(RPS::Rock.find_score(&RPS::Sissors), 7);
	assert_eq!(RPS::Paper.find_score(&RPS::Rock), 8);
	assert_eq!(RPS::Paper.find_score(&RPS::Paper), 5);
	assert_eq!(RPS::Paper.find_score(&RPS::Sissors), 2);
	assert_eq!(RPS::Sissors.find_score(&RPS::Rock), 3);
	assert_eq!(RPS::Sissors.find_score(&RPS::Paper), 9);
	assert_eq!(RPS::Sissors.find_score(&RPS::Sissors), 6);
}


fn part1(input: &str) -> u32 {
	input
		.trim()
		.lines()
		.map(|r#match| {
			let mut battle = r#match.split(" ");

			let opponent = match battle.next().unwrap() {
				"A" => RPS::Rock,
				"B" => RPS::Paper,
				"C" => RPS::Sissors,
				_ => panic!("something fucked up idk"),
			};

			let player = match battle.next().unwrap() {
				"X" => RPS::Rock,
				"Y" => RPS::Paper,
				"Z" => RPS::Sissors,
				_ => panic!("something fucked up idk"),
			};

			player.find_score(&opponent)
		})
		.inspect(|f| {
			println!("{:?}", f);
		}).sum()
}

fn part2(input: &str) -> i32 {
	todo!()
}

fn main() {
	println!("Part 1 Answer: {:?}", part1(input::INPUT));
	println!("Part 1 Answer: {:?}", part2(input::INPUT));
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn part1_test() {
		assert_eq!(
			part1(
				"A Y
B X
C Z"
			),
			15
		)
	}

	#[test]
	fn part2_test() {
		assert_eq!(part2(""), 0)
	}
}
