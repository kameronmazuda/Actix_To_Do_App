use actix_web::{web, HttpResponse};
use crate::diesel;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::schema::to_do;
use crate::json_ser::{
	to_do_item::ToDoItem, 
	to_do_items::ToDoItems
};
use crate::jwt::JwToken;


pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
	println!("here is the message in the token: {}", token.message);

	let conn = establish_connection();
	
	let items = to_do::table
		.filter(to_do::columns::title.eq(&to_do_item.title));

	let _ = diesel::update(items)
		.set(to_do::columns::status.eq("DONE"))
		.execute(&conn);	
	
	HttpResponse::Ok().json(ToDoItems::get_state())
}
