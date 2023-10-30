#[macro_use] extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer};
use actix_service::Service; 
use actix_cors::Cors;

mod views;
mod config;
mod to_do;
mod json_ser;
mod jwt;
mod schema;
mod database;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
		
		println!("running on 127.0.0.1:8080");
		
		let app = App::new()
		.wrap_fn(|req, srv| {
			println!("{}-{}", req.method(), req.uri());
			let future = srv.call(req);
			async {
				let result = future.await?;
				Ok(result)
			}
		})
		.configure(views::views_factory).wrap(cors);
		return app;
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
