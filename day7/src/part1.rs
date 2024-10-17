#![allow(unused)]

enum Filetree {
	// datatype to represent the filetree
	Directory {
		dir_name: String,
		sub_directory: Option<Box<Filetree>>,
	},
	File {
		filename: String,
		size: u32,
	},
}

pub fn main(input: &str) -> u32 {
	let mut input = input.lines();
	let mut cwd: &str = "";

	// represent the entire tree as a recursive type
	let mut filetree: Filetree;

	for line in input {
		if line.starts_with("$ cd ") {
			let dir = line.get(5..).unwrap();
			filetree = Filetree::Directory {
				dir_name: dir.to_string(),
				sub_directory: None,
			};
			cwd = dir;
		} else if line.starts_with("$ ls") {
		}

		println!("{}\n", line);
	}

	todo!()
}
