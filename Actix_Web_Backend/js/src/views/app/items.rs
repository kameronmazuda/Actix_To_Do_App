use actix_web::HttpResponse;
use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
	let html_string = read_file("./templates/main.html".to_string());
	let js_string = read_file("./js/main.js".to_string());

	let html_string = html_string.replace("{{JS}}", &js_string);

	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(html_string)
}