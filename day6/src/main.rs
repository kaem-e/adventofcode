#![allow(unused)]

use std::collections::HashSet;

mod input;

fn part1(input: &str) -> usize {
	input
		// i am literally the smartest person alive i love this fucking solution so much asldfjhakshdflakjdfhlaskjdfh
		.as_bytes()
		.windows(4)
		.position(|window| window.iter().collect::<HashSet<_>>().len() == 4)
		.unwrap() + 4
}

fn part2(input: &str) -> usize {
	input
		.as_bytes()
		.windows(14)
		.position(|window| window.iter().collect::<HashSet<_>>().len() == 14)
		.unwrap() + 14
}

fn main() {
	println!("Part 1 Answer: {:?}", part1(input::INPUT));
	println!("Part 2 Answer: {:?}", part2(input::INPUT));
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn part1_test() {
		assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
		assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
		assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
		assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
		assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
	}

	#[test]
	fn part2_test() {
		assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
		assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
		assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
		assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
		assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
	}
}
