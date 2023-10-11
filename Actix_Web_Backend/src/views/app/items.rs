use actix_web::HttpResponse;
use super::content_loader::{add_component, read_file};

pub async fn items() -> HttpResponse {
	let html_string = read_file("./templates/main.html");
	let js_string = read_file("./js/main.js");
	let css = read_file("./styles/main.css"); 
	let base_css = read_file("./styles/base.css");


	let html_string = html_string.replace("{{JS}}", &js_string);
	let html_string = html_string.replace("{{CSS}}", &css);
	let html_string = html_string.replace("{{BASE_CSS}}", &base_css);

	let html_string = add_component(String::from("header"), html_string);

	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(html_string)
}
