use serde_json::{Map, Value, json};

use crate::state::write_to_file;
use crate::FILE_PATHNAME;

pub trait Create {
	fn create(&self, title: &str, status: &str, state: &mut Map<String, Value>) {
		state.insert(title.to_string(), json!(status));
		write_to_file(FILE_PATHNAME, state);
		println!("\n\nfile {} has been created\n\n", title);
	}
}