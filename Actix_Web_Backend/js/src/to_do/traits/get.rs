use serde_json::{Map, Value};

pub trait Get {
	fn get(&self, title: &str, state: &Map<String, Value>) {
		let item: Option<&Value> = 	state.get(title);
		match item {
			Some(res) => {
				println!("\n\nItem: {}", title);
				println!("Status: {}\n\n", res);
			},
			None => println!("item {} could not be found.", title)
		}
	}
}