#![allow(unused)]

pub mod input;

pub fn part1(input: &str) -> i32 {
	input
		.trim()
		.split("\n\n")
		.map(|chunk| -> i32 {
			chunk
				.lines()
				.map(|line| line.trim().parse::<i32>().unwrap())
				// .inspect(|f| println!("parsed: {:?}", f))
				.sum::<i32>()
		})
		// .inspect(|f| println!("Chunk sum: {:?}\n\n", f))
		.max()
		.unwrap()
}

pub fn part2(input: &str) -> i32 {
	let mut temp: Vec<i32> = input
		.trim()
		.split("\n\n")
		.map(|chunk| -> i32 {
			chunk
				.lines()
				.map(|line| line.trim().parse::<i32>().unwrap())
				.sum::<i32>()
		})
		.collect::<Vec<i32>>();

	temp.sort_by(|a, b| b.cmp(a));
	let sum = temp[0..=2].iter().sum();

	sum
}

fn main() {
	println!("Part 1 Solution: {}", part1(input::inputs::INPUT_PART1));
	// usually they have two different inputs but we in this case they didnt idk
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
