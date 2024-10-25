#![allow(unused)]

mod input;

use itertools::Itertools;

fn part1(input: &str) -> u32 {
	let input = input
		.lines()
		.map(|x| {
			x.chars()
				.filter_map(|f| f.to_digit(10))
				.collect::<Vec<u32>>()
		})
		.collect::<Vec<Vec<u32>>>();

	let joined = input
		.into_iter()
		.map(|x| {
			let joined = x.first().unwrap() * 10
				+ match x.last() {
					Some(last) => last,
					None => x.first().unwrap(),
				};
			joined
		})
		.collect::<Vec<u32>>();

	let sum = joined.into_iter().sum::<u32>();

	return sum;
	// VERY old solution lmao need to redo this after 2022
	// ngl i could probably make this way more efficient but eh
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
				"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
			),
			142
		)
	}

	#[test]
	fn part2_test() {
		assert_eq!(
			part2(
				"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
			),
			0
		)
	}
}
