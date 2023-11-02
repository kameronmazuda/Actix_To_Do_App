use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse, Responder};
use actix_web::HttpResponseBuilder;

use crate::database::DB;
use crate::json_ser::new_user::NewUserSchema;
use crate::models::user::new_user::NewUser;
use crate::schema::users;

pub async fn create(new_user: web::Json<NewUserSchema>, db: DB) -> impl Responder {
    
    let new_user = NewUser::new(
        new_user.name.clone(),
        new_user.email.clone(),
        new_user.password.clone()
        );

    let insert_result = diesel::insert_into(users::table).values(&new_user).execute(&db.connection);

    match insert_result {
        Ok(_) => HttpResponse::Created(), // return 201 Response if user was inserted succesfully
        Err(_) => HttpResponse::Conflict() // If user exists return 409 error 
    }
}
