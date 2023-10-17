use crate::diesel;
use diesel::prelude::*;
use actix_web::{HttpRequest, HttpResponse};

use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::json_ser::to_do_items::ToDoItems;

pub async fn create(req: HttpRequest) -> HttpResponse {
	let title: String = req.match_info().get("title").unwrap().to_string();
	
	let conn = establish_connection();
	
	let items = to_do::table
		.filter(to_do::columns::title.eq(&title.as_str()))
		.order(to_do::columns::id.asc())
		.load::<Item>(&conn)
		.unwrap();

	if items.len() == 0 {
		let new_post = NewItem::new(title);
		let _ = diesel::insert_into(to_do::table).values(&new_post).execute(&conn);
	}

	HttpResponse::Ok().json(ToDoItems::get_state())
}
