#![allow(unused)]

use std::fmt::{Debug, Display};

pub fn part1(input: &str) -> i32 {
	todo!()
}

pub fn part2<T: Display>(input: String) -> T {
	todo!()
}

fn main() {
	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_test() {
		assert_eq!(
			part1("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"),
			24000
		);
	}

	#[test]
	fn part2_test() {
		panic!()
	}
}
