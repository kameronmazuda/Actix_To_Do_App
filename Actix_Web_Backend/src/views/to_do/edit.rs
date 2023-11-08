use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::database::DB;
use crate::schema::to_do;
use crate::json_ser::{
	to_do_item::ToDoItem, 
	to_do_items::ToDoItems
};
use crate::jwt::JwToken;

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken, db: DB) -> HttpResponse {

	let items = to_do::table
		.filter(to_do::columns::title.eq(&to_do_item.title))
		.filter(to_do::columns::user_id.eq(&token.user_id));

	let _ = diesel::update(items)
		.set(to_do::columns::status.eq("DONE"))
		.execute(&db.connection);	
	
	HttpResponse::Ok().json(ToDoItems::get_state(token.user_id))
}
