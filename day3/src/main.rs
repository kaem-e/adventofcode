#![allow(unused)]

mod input;

use std::{any::Any, collections::HashSet};

use itertools::Itertools;
use rayon;

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
	input
		.lines()
		.chunks(3)
		.into_iter()
		.map(|chunk| {
			chunk
				.map(|line| line.chars().collect::<HashSet<_>>())
				.reduce(|a, b| a.intersection(&b).cloned().collect())
				.filter(|hs| !hs.is_empty())
		})
		.count() as u32
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
