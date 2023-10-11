use std::fs;

pub fn read_file(file_path: String) -> String {
	let contents: String = fs::read_to_string(file_path)
	.expect("Failed to read file.");

	contents
}