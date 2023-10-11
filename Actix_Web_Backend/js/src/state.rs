use std::fs::File;
use std::fs;
use std::io::Read;

use serde_json::{Map, Value, json};

pub fn read_file(file_name: &str) -> Map<String, Value> {
	let mut file = File::open(file_name.to_string()).unwrap();
	
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	
	let json: Value = serde_json::from_str(&data).unwrap();
	let state: Map<String, Value> = json.as_object().unwrap().clone();
	
	state
}

pub fn write_to_file(
	file_name: &str, 
	new_state: &mut Map<String, Value>)
{
	fs::write(
		file_name.to_string(), 
		json!(new_state).to_string()
	).expect("Unable to write to file.")
}
