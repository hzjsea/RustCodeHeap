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
use std::{env, fmt};
use route::*;

mod models;
mod db;
mod schema;
mod route;

fn rocket() -> rocket::Rocket {
    // 加载目录中的
    // env::vars 只会加载系统的环境变量 ，但是dotenv.ok设置之后会在加载当前目录中.env中的环境变量的设置
    dotenv().ok();
    for (index, value ) in env::vars(){
        println!("{}{}",index,value)
    }

    // 读取环境变量
    let database_url = env::var("DATABASE_URL").expect("msg");

    // 创建 mysql 连接池
    let pool = init_pool(database_url);
    
    rocket::ignite()
        .manage(pool) // 安装连接池
        .mount(  // 挂载
            "/users/", 
            routes![get_all,auth_data])
        .mount(
            "/", 
            routes![index])
}

fn main(){
    let rocket = rocket(); 
    rocket.launch(); // 点燃
}

