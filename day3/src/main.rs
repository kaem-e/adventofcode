mod input;

use std::collections::HashSet;

fn part1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| -> u32 {
			let chunk_len = line.len() / 2;
			let (head, tail) = line.split_at(chunk_len);

			head
				.chars()
				.find(|&x| tail.contains(x))
				.map(|f| match f {
					'a'..='z' => (f as u8 - b'a' + 1) as u32,
					'A'..='Z' => (f as u8 - b'A' + 27) as u32,
					_ => panic!("tried parsing non-alphanumeric char: {:?}", f),
				})
				.unwrap()
		})
		.sum::<u32>()
}

fn part2(input: &str) -> u32 {
	let mut input = input.lines();
	let mut intersection: char = ' ';
	let mut sum: u32 = 0;

	let mut compute_closure = || -> Option<char> {
		let line1: HashSet<char> = HashSet::from_iter(input.next()?.chars());
		let line2: HashSet<char> = HashSet::from_iter(input.next()?.chars());
		let line3: HashSet<char> = HashSet::from_iter(input.next()?.chars());
		intersection = line1
			// this inefficient as fuck fr
			.intersection(&line2)
			.cloned()
			.collect::<HashSet<char>>()
			.intersection(&line3)
			.cloned()
			.collect::<HashSet<char>>()
			.iter()
			.next()
			.unwrap()
			.to_owned();
		Some(intersection)
	};

	while let Some(a) = compute_closure() {
		sum += match a {
			'a'..='z' => (a as u8 - b'a' + 1) as u32,
			'A'..='Z' => (a as u8 - b'A' + 27) as u32,
			_ => panic!("tried parsing non-alphanumeric char: {:?}", a),
		};
	}

	sum
}

fn main() {
	println!("Part 1 Answer: {:?}", part1(input::INPUT.trim()));
	println!("Part 1 Answer: {:?}", part2(input::INPUT.trim()));
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn part1_test() {
		assert_eq!(
			part1(
				"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
					.trim()
			),
			157
		)
	}

	#[test]
	fn part2_test() {
		assert_eq!(
			part2(
				"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
					.trim()
			),
			70
		)
	}
}
