#![allow(unused)]
use itertools::Itertools;
use std::cmp::Reverse;
mod input;

fn part1(input: &str) -> i32 {
	input
		.split("\n\n")
		.map(|group| -> i32 {
			group
				.lines()
				.map(|line| line.parse::<i32>().unwrap())
				.sum::<i32>()
		})
		.max()
		.unwrap()
}

fn part2(input: &str) -> u32 {
	input
		.lines()
		.map(|v| v.parse::<u32>().ok())
		.batching(|it| it.map_while(|x| x).sum1::<u32>())
		.map(Reverse)
		.k_smallest(3)
		.map(|x| x.0)
		.sum::<u32>()
}

fn main() {
	println!("Part 1 Solution: {}", part1(input::inputs::INPUT_PART1));
	println!("Part 2 Solution: {}", part2(input::inputs::INPUT_PART1));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_test() {
		assert_eq!(
			part1(
				"1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
			),
			24000
		);
	}

	#[test]
	fn part2_test() {
		assert_eq!(
			part2(
				"1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
			),
			45000
		)
	}
}
