#![allow(unused)]
mod input;
use itertools::{self, Itertools};

fn part1(input: &str) -> u32 {
	input
		.lines()
		.filter(|&line| {
			let mut line = line.split(",");
			let mut compute = |line: &mut std::str::Split<'_, &str>| {
				line
					.next()
					.unwrap()
					.split("-")
					.map(|num| num.parse::<u32>().unwrap())
					.collect_tuple()
					.unwrap()
			};
			let (f_start, f_end) = compute(&mut line);
			let (s_start, s_end) = compute(&mut line);
			let res = ((f_start <= s_start && f_end >= s_end)
				|| (s_start <= f_start && s_end >= f_end));
			res
		})
		.count() as u32
}

fn part2(input: &str) -> u32 {
	input
		.lines()
		.filter(|&line| {
			let mut line = line.split(",");
			let mut compute = |line: &mut std::str::Split<'_, &str>| {
				line
					.next()
					.unwrap()
					.split("-")
					.map(|num| num.parse::<u32>().unwrap())
					.collect_tuple()
					.unwrap()
			};
			let (f_start, f_end) = compute(&mut line);
			let (s_start, s_end) = compute(&mut line);

			let res = (f_start <= s_end && f_end >= s_start)
				|| (s_start <= f_end && s_end >= f_start);
			res
		})
		.count() as u32
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
				"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
			),
			2
		)
	}

	#[test]
	fn part2_test() {
		assert_eq!(
			part2(
				"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
			),
			4
		)
	}
}
