#![feature(decl_macro)]


#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use db::init_pool;
use dotenv::dotenv;
use std::env;
use route::*;

mod models;
mod db;
mod schema;
mod route;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("msg");

    let pool = init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount(
            "/users/", 
            routes![get_all,auth_data])
        .mount(
            "/", 
            routes![index])
}

fn main(){
    let rocket = rocket();
    rocket.launch();
}

