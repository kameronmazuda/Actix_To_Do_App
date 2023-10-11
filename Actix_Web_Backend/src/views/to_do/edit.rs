use actix_web::{web, HttpResponse};
use serde_json::{Value, Map};

use crate::state::read_file;
use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::json_ser::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::processes::process_input;
use crate::jwt::JwToken;
use crate::FILE_PATHNAME;

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
	let state: Map<String, Value> = read_file(FILE_PATHNAME);
	let status: TaskStatus;	
	
	println!("here is the message in the token: {}", token.message);
	
	match &state.get(&to_do_item.title) {
		Some(val) => status = TaskStatus::from_string(val.as_str().unwrap().to_string()),
		None =>  return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title))
	}
	
	let item = to_do_factory(&to_do_item.title, status.clone()); 
	
	if status.stringify() == TaskStatus::from_string(to_do_item.status.as_str().to_string()).stringify() {
		return HttpResponse::Ok().json(ToDoItems::get_state())
	}
	
	process_input(item, "edit", &state);
	
	HttpResponse::Ok().json(ToDoItems::get_state())
}
