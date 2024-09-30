#![allow(unused)]
// entirely separate file because i waana try doing this with a few diff datastructures & shit

// object oriented my fucking ass this shits so bad

use itertools::Itertools;

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
/// // note this isnt proper rust syntax im just using this for represntation
/// Stack: Vec {
/// 	Vec: [Z, N],
/// 	Vec: [M, C, D],
/// 	Vec: [P],
/// }
/// ```
/// Avalible methods are
/// - `new`: creates the struct by parsing the string lines iterator

struct Stack(Vec<Vec<char>>);
impl Stack {
	/// takes in an iterator of all the lines that repersent the cargo stack
	/// and craetes a new `Stack` with a 2D vector representing the arrangment of the entire vector
	fn new_by_parse(mut lines: std::str::Lines) -> Stack {
		let askdfja = lines.clone();
		for line in askdfja {
			println!("{line}");
		}
		lines.next_back(); // remove the last line with all the numbers
		let mut parentvec: Vec<Vec<char>> = Vec::new();
		// let mut parentvec: Vec<Vec<char>> = vec![vec![], vec![], vec![], vec![], vec![], vec![]];

		lines.for_each(|line| {
			let line = line
				.replace("] [", "")
				.replace("[", "")
				.replace("]", "")
				// this is cheating tbh but im just so done with this problem for now
				.replace("    ", ".")
				.replace(" ", "")
				.replace(".", " ");
			println!("\n\n\n start: ;{line}; end");

			let temp = line.char_indices().filter_map(|(x, y)| {
				if y.is_alphabetic() {
					Some((x, y))
				} else {
					None
				}
			});

			for (index, char) in temp {
				println!("\npushing: {char} at index: {index}");
				if let Some(internalvec) = parentvec.get_mut(index) {
					internalvec.push(char);
				} else {
					// skip colums that already exist
					let len = parentvec.len();
					println!(" vec has up to index: {}", ((len as i8) - 1));
					for x in 0..=index {
						if len <= x {
							println!("\tcreating new vec at index: {x}");
							parentvec.push(Vec::new());
						} else {
							println!("\tskipping creating new vec at index: {x}, already exists");
						}
					}
					parentvec.get_mut(index).unwrap().push(char);
				};
			}
		});

		// make the last element represent whats on top
		for internal in parentvec.iter_mut() {
			internal.reverse();
		}
		println!("\n\nparentvec: {:?}\n\n", parentvec);
		Stack(parentvec)
	}

	/// method on any stack that takes in commands as an argument
	/// where commands are a iterator type `Lines`
	fn sort(mut self, mut commands: std::str::Lines) -> Self {
		for command in commands {
			// destruct everything
			let (_move, ammount, _from, from, _to, to) =
				command.split_whitespace().collect_tuple().unwrap();
			let ammount = (ammount.parse::<usize>().unwrap());
			let from = (from.parse::<usize>().unwrap() - 1);
			let to = (to.parse::<usize>().unwrap() - 1);
			println!("{:?}", (_move, ammount, _from, from, _to, to));

			// rearrange the vector
			// thisll reverse the direction but thats how the parser instructs it be done so
			for x in 0..ammount {
				let from = self.0.get_mut(from).unwrap();
				let from_char = from.pop().unwrap();
				let to = self.0.get_mut(to).unwrap();
				to.push(from_char);
			}
		}
		self
	}

	/// creates an answer string by parsing the lastmost crates in the vector
	fn get_answer(&self) -> String {
		let mut answer = String::new();

		for vec in self.0.iter() {
			let topchar = vec.last().unwrap();
			answer.push(topchar.clone());
		}

		answer
	}
}

/// Main entry point for the entire module
/// takes in the input, returns the result
pub fn main(input: &str) -> String {
	let (stack, commands) = input.split_once("\n\n").unwrap();
	let answer = Stack::new_by_parse(stack.lines())
		.sort(commands.lines())
		.get_answer();
	return answer;
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn part1_test() {
		assert_eq!(
			main(
				"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
			),
			"CMZ"
		)
	}
}
