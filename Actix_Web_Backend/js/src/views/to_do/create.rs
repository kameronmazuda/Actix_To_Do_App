use serde_json::{Value, Map};
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::state::read_file;
use crate::processes::process_input;
use crate::json_ser::to_do_items::ToDoItems;

use crate::FILE_PATHNAME;

#[derive(Deserialize)]
pub struct Body {
	pub title: String,
}

pub async fn create(body: web::Json<Body>) -> HttpResponse {
	let state: Map<String, Value> = read_file(FILE_PATHNAME);
	
	if let Some(_val) = &state.get(&body.title) {
		return HttpResponse::NotFound().json(format!("{} already exists.", &body.title));
	}

	let item = to_do_factory(&body.title.as_str(), TaskStatus::PENDING);

	process_input(item, "create", &state);

	HttpResponse::Ok().json(ToDoItems::get_state())
}