use actix_web::{
    body::BoxBody, http::header::ContentType,
    HttpRequest, HttpResponse, Responder,
};
use serde::Serialize;
use crate::diesel;
use crate::jwt::JwToken;
use diesel::prelude::*;

use crate::database::{establish_connection, DBCONNECTION};
use crate::models::item::item::Item;
use crate::schema::to_do; 

use crate::to_do::TaskTypes;
use crate::to_do::structs::base::Base;
use crate::to_do::{to_do_factory, enums::TaskStatus};

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
	
	pub fn get_state(user_id: i32) -> ToDoItems {
		
		let conn = DBCONNECTION.db_connection.get().unwrap();
		
		let items = to_do::table
			.filter(to_do::columns::user_id.eq(&user_id))
			.order(to_do::columns::id.asc())
			.load::<Item>(&conn)
			.unwrap();
	
		let mut array_buffer = Vec::new();

		for item in items {
			let status = TaskStatus::from_string(item.status.as_str().to_string());
			let item = to_do_factory(&item.title, status);
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






















