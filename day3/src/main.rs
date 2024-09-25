#![allow(unused)]

mod input;
// mod priority;

fn part1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| -> u32 {
			println!("{}", &line);
			let chunk_len = line.len() / 2;
			let (mut arr_head, mut arr_tail) =
				(Vec::with_capacity(chunk_len), Vec::with_capacity(chunk_len));
			let mut char_iterator = line.chars();

			for _ in 0..chunk_len {
				arr_head.push(char_iterator.next().unwrap());
			}
			for _ in 0..chunk_len {
				arr_tail.push(char_iterator.next().unwrap());
			}


			println!("{:?}", &arr_head);
			println!("{:?}", &arr_tail);

			// rust binary search seems to be utterly stupid
			arr_head.sort();
			arr_tail.sort();

			let mut found_char: char = ' ';
			// there HAS to be a more efficient way of doing this bru
			for x in arr_head {
				// println!("{}", &x);
				found_char = if arr_tail.binary_search(&x).is_ok() {
					x
				} else {
					continue;
				}
			}
			println!("{}\n", &found_char);

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
