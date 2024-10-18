mod input;
mod types;
use types::{NeededResult, PlayerChoice};

fn part1(input: &str) -> u32 {
	input
		.trim()
		.lines()
		.map(|lines| {
			let mut battle = lines.split(" ");

			let opponent = battle
				.next()
				.unwrap()
				.parse::<PlayerChoice>()
				.expect("screwed up while parsing");
			let player = battle
				.next()
				.unwrap()
				.parse::<PlayerChoice>()
				.expect("screwed up while parsing");

			player.find_score(&opponent)
		})
		.inspect(|f| {
			println!("{:?}", f);
		})
		.sum()
}

fn part2(input: &str) -> u32 {
	input
		.trim()
		.lines()
		.map(|lines| {
			let mut battle = lines.split(" ");

			let opponent = battle
				.next()
				.unwrap()
				.parse::<PlayerChoice>()
				.expect("screwed up while parsing");
			let player = battle
				.next()
				.unwrap()
				.parse::<PlayerChoice>()
				.expect("screwed up while parsing");

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
