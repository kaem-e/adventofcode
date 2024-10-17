#![allow(unused)]
mod input;
mod part1;
mod part2;


use part1::main as part1;
use part2::main as part2;

fn main() {
	println!("Part 1 Answer: {:?}", part1(input::INPUT));
	println!("Part 2 Answer: {:?}", part2(input::INPUT));
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn part1_test() {
		assert_eq!(part1("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"), 95437)
	}

	#[test]
	fn part2_test() {
		assert_eq!(part2(""), 0)
	}
}
