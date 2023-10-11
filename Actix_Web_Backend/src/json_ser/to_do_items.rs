use serde::Serialize;
use crate::to_do::TaskTypes;
use crate::to_do::structs::base::Base;
use crate::state::read_file;
use crate::to_do::{to_do_factory, enums::TaskStatus};

use serde_json::{Value, Map};
use actix_web::{
    body::BoxBody, http::header::ContentType,
    HttpRequest, HttpResponse, Responder,
};

use crate::FILE_PATHNAME;

#[derive(Serialize)]
pub struct ToDoItems {
	pub pending_items: Vec<Base>,
	pub done_items: Vec<Base>,
	pub pending_item_count: i8,
	pub done_item_count: i8,
}

impl ToDoItems {
	pub fn new(tasks: Vec<TaskTypes>) -> ToDoItems {
		// sorting tasks by type
		let mut pending_array_buffer = Vec::new();
		let mut done_array_buffer = Vec::new();
		
		for task in tasks {
			match task {
				TaskTypes::Pending(val) => pending_array_buffer.push(val.super_struct),
				TaskTypes::Done(val) => done_array_buffer.push(val.super_struct),
			}
		}
		
		// count the num of pending and done tasks
		let pending_count = pending_array_buffer.len() as i8;
		let done_count = done_array_buffer.len() as i8;
	
		Self {
			pending_items: pending_array_buffer,
			done_items: done_array_buffer,
			pending_item_count: pending_count,
			done_item_count: done_count,
		}
	}
	
	pub fn get_state() -> ToDoItems {
		let state: Map<String, Value> = read_file(FILE_PATHNAME); 
	
		let mut array_buffer = Vec::new();
		
		for (key, value) in state {
			let status = TaskStatus::from_string(value.as_str().unwrap().to_string());
			let item: TaskTypes = to_do_factory(&key, status);
			array_buffer.push(item);
		}
		
		ToDoItems::new(array_buffer)
		
	}
}

impl Responder for ToDoItems {
	type Body = BoxBody;
	
	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		let body = serde_json::to_string(&self).unwrap();
		HttpResponse::Ok()
			.content_type(ContentType::json())
			.body(body)
	}
}






















