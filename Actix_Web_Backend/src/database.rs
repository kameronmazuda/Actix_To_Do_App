use diesel::prelude::*;
use diesel::{r2d2::{Pool, ConnectionManager, PooledConnection}}
use diesel::pg::PgConnection;
use crate::config::Config;
use actix_web::dev::Payload;
use actix::error::ErrorServiceUnavailable;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future{Ready, ok, err};
use lazy_static::lazy_static;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_connection: PgPool;
}

pub struct DB {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>
}

impl FromRequest for DB {
    type Error = Error;
    type Future = Ready<Result<DB, Error>>;

    fn from_request(_: &HttpRequest, _: &mut Payload) -> Self::Future{
        {
            match DBCONNECTION.db_connection.get() {
                Ok(connection) => {
                    return ok(DB{connection});
                },
                Err(_) => {
                    return err(ErrorServiceUnavailable("could not make connection to database");
                }

            }
        }
    }
}


lazy_static! {
    pub static ref DBCONNECTION: DbConnection = {
        let connection_string = Config::new().map.get("DB_URL").unwrap().as_str().unwrap().to_string(); 

        DbConnection {
            db_connection: PgPool::builder().max_size(8)
                .build(ConnectionManager::new(connection_string))
                .expect("failed to create db connection pool")
        }
    }
}
pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    return DBCONNECTION.db_connection().get().unwrap();
}



