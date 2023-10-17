use actix_web::{web, HttpResponse};
use crate::diesel;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::json_ser::{
	to_do_item::ToDoItem, 
	to_do_items::ToDoItems
};
use crate::jwt::JwToken;


pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
	println!("here's the message in the token: {}", token.message);
	
	let conn = establish_connection();
	
	let items = to_do::table
		.filter(to_do::columns::title.eq(&to_do_item.title))
		.order(to_do::columns::id.asc())
		.load::<Item>(&conn)
		.unwrap();
	
	let _ = diesel::delete(&items[0]).execute(&conn);
	
	HttpResponse::Ok().json(ToDoItems::get_state())
}
