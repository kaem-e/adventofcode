#![allow(unused)]

mod input;

fn part1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| -> u32 {
			let chunk_len = line.len() / 2;
			let (head, tail) = line.split_at(chunk_len);

			// there HAS to be a more efficient way of doing this bru
			let mut found_char: char = ' ';
			for x in head.chars() {
				found_char = match tail.find(x) {
					Some(_) => x,
					None => continue,
				};
			}
			// println!("Found: {}\n", &found_char);

			match found_char {
				'a'..='z' => ((found_char as u8) - b'a') + 1,
				'A'..='Z' => ((found_char as u8) - b'A') + 27,
				x => panic!("tried parsing non_alphanumeric char: {:?}", x),
			}
			.into()
		})
		.sum::<u32>()
}

fn part2(input: &str) -> u32 {
	todo!()
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
			0
		)
	}
}
