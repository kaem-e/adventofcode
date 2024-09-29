#![allow(unused)]
mod input;
mod part1;

pub fn part2(input: &str) -> u32 {
	todo!()
}

fn main() {
	println!("Part 1 Answer: {:?}", part1::call(input::INPUT));
	println!("Part 1 Answer: {:?}", part2(input::INPUT));
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn part2_test() {
		assert_eq!(part2(""), 0)
	}
}
