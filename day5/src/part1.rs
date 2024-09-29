#![allow(unused)]
// entirely separate file because i waana try doing this with a few diff datastructures & shit

/// simple struct that just repsents each command
struct Command {
	number: u8,
	from: u8,
	to: u8,
}

/// Represents a stack as a 2D vector
/// column's themselves are in a parent struct
/// The individual column's from the string will be represented by vectors of strings. i.e.
/// ```
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
/// ```
/// from the input will be repsented as
/// ```
/// TODO
/// ```
/// Avalible methods are
/// - `new`: creates the struct by parsing the string lines iterator

struct Stack(Vec<Vec<char>>);
impl Stack {
	fn new_by_parse(mut lines: std::str::Lines) -> Stack {
		lines.next_back(); // remove the last line with all the numbers
		let parentvec: Vec<Vec<char>> = Vec::new();
		for line in lines {
			// i am a fucking genius were so back
			let test: Vec<(usize, char)> = line
				.replace("", "")
				.trim
				.char_indices()
				.filter_map(|(x, y)| {
					if y.is_alphabetic() {
						Some((x, y))
					} else {
						None
					}
				})
				.collect();
			println!("Got: {:?}", test)
		}

		todo!();
		Stack(parentvec)
	}
}

/// Main entry point for the entire module
/// takes in the input, returns the result
pub fn call(input: &str) -> String {
	let (stack, commands) = input.split_once("\n\n").unwrap();
	let stack = Stack::new_by_parse(stack.lines());

	todo!()
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn part1_test() {
		assert_eq!(
			call(
				"    [D]    
[N] [C]    
[Z] [M] [P]
[Z] [M] [P] [A]
[Z] [M] [P] [A] [B]
[Z] [M] [P] [A] [B] [C]
 1   2   3   4   5   6

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
			),
			"CMZ"
		)
	}
}
