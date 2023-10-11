use serde_json::{Map, Value, json};
use crate::state::write_to_file;
use super::super::enums::TaskStatus;
use crate::FILE_PATHNAME;

pub trait Edit {
	fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
		state.insert(
			title.to_string(), 
			json!(TaskStatus::DONE.stringify())
		);
		write_to_file(FILE_PATHNAME, state);
		println!("task {} is set to done.", title);
	}
	fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
		state.insert(
			title.to_string(), 
			json!(TaskStatus::PENDING.stringify())
		);
		write_to_file(FILE_PATHNAME, state);
		println!("task {} is set to pending.", title);
	}
}