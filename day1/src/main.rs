pub mod input;

pub fn part1(input: &str) -> i32 {
	input
		.split("\n\n")
		.map(|chunk| -> i32 {
			chunk
				.lines()
				.map(|line| line.parse::<i32>().unwrap())
				.sum::<i32>()
		})
		.max()
		.unwrap()
}

pub fn part2(input: &str) -> u32 {
	let mut max_nums: [u32; 3] = [u32::MIN; 3];
	input
		.trim()
		.split("\n\n")
		// .inspect(|f| println!("About to sum: \n{f}"))
		.for_each(|chunk| {
			let sum = chunk
				.lines()
				.map(|line| line.trim().parse::<u32>().unwrap())
				.sum::<u32>();
			// println!("Result: {sum}\n");
			if max_nums[0] <= sum {
				max_nums[0] = sum;
				// this feels really inefficient ngl,
				// thisll have to be called for every element
				// but atleast its only being done when a new element is added so
				max_nums.sort();
			}
		});

	max_nums.iter().sum()
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
