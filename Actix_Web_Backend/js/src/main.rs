use actix_web::{App, HttpServer};
use actix_service::Service; 

mod views;
mod to_do;
mod state;
mod processes;
mod json_ser;
mod jwt;

pub const FILE_PATHNAME: &str = "./state.json";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		println!("running on 127.0.0.1:8080");
		let app = App::new()
		.wrap_fn(|req, srv| {
			println!("{:?}", req);
			let future = srv.call(req);
			async {
				let result = future.await?;
				Ok(result)
			}
		})
		.configure(views::views_factory);
		return app;
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}

/*
use serde_json::{Map, Value};

use state::read_file;
use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use processes::process_input;

fn main() {
	let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    
	let state: Map<String, Value> = read_file(FILE_PATHNAME);
    let status: String;
    
	match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None=> {
            status = "pending".to_owned();
        }
    }
    let item = to_do_factory(title,
                             TaskStatus::from_string(
                             status.to_uppercase()));
    process_input(item, command, &state);
}
*/
