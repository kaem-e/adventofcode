mod input;
mod types;
use types::{rps, NeededResult};

fn part1(input: &str) -> u32 {
	input
		.trim()
		.lines()
		.map(|lines| {
			let mut battle = lines.split(" ");

			let opponent = match battle.next().unwrap() {
				"A" => rps::Rock,
				"B" => rps::Paper,
				"C" => rps::Sissors,
				_ => panic!("something fucked up idk"),
			};

			let player = match battle.next().unwrap() {
				"X" => rps::Rock,
				"Y" => rps::Paper,
				"Z" => rps::Sissors,
				_ => panic!("something fucked up idk"),
			};

			player.find_score(&opponent)
		})
		// .inspect(|f| {
		// 	println!("{:?}", f);
		// })
		.sum()
}

fn part2(input: &str) -> u32 {
	input
		.trim()
		.lines()
		.map(|lines| {
			let mut battle = lines.split(" ");

			let opponent = match battle.next().unwrap() {
				"A" => rps::Rock,
				"B" => rps::Paper,
				"C" => rps::Sissors,
				_ => panic!("something fucked up idk"),
			};

			let player = match battle.next().unwrap() {
				"X" => NeededResult::Lose,
				"Y" => NeededResult::Draw,
				"Z" => NeededResult::Win,
				_ => panic!("something fucked up idk"),
			};

			player.find_score(&opponent)
		})
		.sum()
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
		assert_eq!(
			part2(
				"A Y
B X
C Z"
			),
			12
		)
	}
}
