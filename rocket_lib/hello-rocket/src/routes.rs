use diesel::{self, prelude::*};
use rocket_contrib::json::Json;
use crate::models::users;
use crate::schema;
use crate::DbConn;


#[get("/user_get_all")]
pub fn user_get_all(conn: DbConn) -> String{
    format!("{}",users.id)
}
