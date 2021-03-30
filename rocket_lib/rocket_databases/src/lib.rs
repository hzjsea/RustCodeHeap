#[marco_use]
extern crate diesel;
extern crate dotenv;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;


pub mod scheme;
pub mod models;

pub fn new_connect() -> MysqlConnection{
    dotenv().ok();

    let databases_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&databases_url).expect(&format!("connect error ,{}",databases_url))

}
