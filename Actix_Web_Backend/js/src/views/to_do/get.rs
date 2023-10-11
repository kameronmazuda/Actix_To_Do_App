use actix_web::Responder;
use crate::json_ser::to_do_items::ToDoItems;

pub async fn get() -> impl Responder {
	ToDoItems::get_state()
}
