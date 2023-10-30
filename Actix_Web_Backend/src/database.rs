use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::config::Config;

pub fn establish_connection() -> PgConnection {
    let config = Config::new();
    let database_url = config.map.get("DB_URL").unwrap().as_str().unwrap();
	
	PgConnection::establish(&database_url)
		.unwrap_or_else(
			|_| panic!("Error connecting to {}", database_url)
		)
}


