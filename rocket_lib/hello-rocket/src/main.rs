#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
fn pong() -> &'static str{
    "pong"
}


#[get("/echo")] // 路由
fn echo() -> &'static str{ // request function
    "world" // response 
}

#[get("/user/echo")]
fn user_echo() -> &'static str{
    "world" 
}


// dynamic path
#[get("/user/<name>")]
fn echo_name(name: &rocket::http::RawStr) -> String{
    format!("name\tis\t{}", name.as_str())
}

#[get("/user/<name>/<age>/<cool>")]
fn hello(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}


// Multiple Segments#
use std::{cell::UnsafeCell, hash, iter::successors, ops::Deref, os::macos::raw::stat, path::PathBuf};
#[get("/user/path/<path..>")]
fn mult_path(path: PathBuf) -> String{
    format!("{:?}",path.as_os_str())
}

// Named file
// use rocket::response::NamedFile;
// #[get("/<file..>")]
// fn create_file(file: PathBuf) -> Option<NamedFile> {
//     NamedFile::open(std::path::Path::new("xx.txt").join(file)).ok()
// }


#[get("/user/<id>")]
fn user(id: usize) -> String { 
    format!("id {} type is uszie",id)    
}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String { 
    format!("id {} type is iszie",id)     
}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &rocket::http::RawStr) -> String{
    format!("id {} type is rawStr",id) 
}


// query  

// curl http://localhost:8000/api/query/hello\?name\=hzj\&wave
// #[get("/hello?wave&<name>")]
// fn hello_query(name: &rocket::http::RawStr) -> String {
//     format!("Hello, {}!", name.as_str())
// }

#[get("/hello?wave&<name>")]
fn hello_quert_map(name: Option<String>) -> String {
    name.map(|name| format!("Hi, {}!", name))
        .unwrap_or_else(|| "Hello!".into())
}

/// md5(888888+key)

// struct body


#[derive(FromForm)]
struct User {
    name: String,
    account: usize,
}



//  curl http://localhost:8000/api/query/item\?id\=1\&name\=hzj\&account\=2
#[get("/item?<id>&<user..>")]
fn item_get(id:i32, user:rocket::request:: Form<User>) -> String{
    format!("{}{}{}",id,user.account,user.name)
}


#[derive(Debug)]
struct Auth<'a>{
    authorization: Option<&'a str>,
}

impl <'a ,'r> rocket::request::FromRequest<'a,'r> for Auth<'a> {
    type Error = ();

    fn from_request(request: &'a rocket::request::Request<'r>) ->rocket::request::Outcome<Self, Self::Error> {
       let authorization = request.headers().get("authorization").next();
       println!("{:?}",request);
       println!("{:?}",authorization);
       rocket::Outcome::Success(Auth {
            authorization
        })
    }
}

#[get("/auth")]
fn auth(guard:Auth) -> &str{
    guard.authorization.unwrap_or("")
}


// about cookie

#[get("/cookie")]
fn print_cookie(cookie: rocket::http::Cookies) -> Option<String>{
    cookie.get("name").map(|value| format!("Message {}",value))
}

// use rocket::request::{self, Form};
// #[derive(Debug,FromForm,Responder)]
// struct UserP{
//     name:String,
//     age:i32
// }

// // data send format
// #[post("/user/json_data",format = "application/json", data = "<userp>")]
// fn new_user(userp:Form<UserP>){
//     println!("{:?}",userp)
// }
// // data reve format
// #[get("/user/json_get/<id>",format = "json")]
// fn new_user_get(id:i32) -> Option<UserP>{
//     Some(UserP{
//         name:"hzj".to_owned(),
//         age:21
//     })
// }

use content::Json;
use rocket::{data::DataStream, response::status};

#[get("/<id>")]
fn get_id(id: i32) -> status::Accepted<String>{
    status::Accepted(Some(format!("id: '{}'", id)))
}

use rocket::response::content;
#[get("/content")]
fn get_str() ->  content::Json<&'static str>{
    content::Json("
        {
            'hello':'world'
        }
    ")
}


// status
use rocket::http::Status;

#[get("/status")]
fn just_fail() -> Status {
    Status::NotAcceptable
}

use rocket::http::ContentType;
use rocket::http::Header;

// custom status
// #[derive(Responder)]
// #[response(status = 500, content_type = "json")]
// struct MyResponder {
//     inner: OtherResponder,
//     header: ContentType,
//     more: Header<'static>,
//     #[response(ignore)]
//     unrelated: MyType,
// }




// format data 

// Body data
#[derive(Debug)]
struct People{
    name:String, // 像这样的 需要去顾虑 String &'static str 还是 &'a str 什么时候去顾虑
    age:i32
}

impl People {
    fn new(name:String,age:i32) -> Self{
        People{
            name:name,
            age:age
        }
    }
}

use rocket::data::FromDataSimple;
use rocket::request::Request;
use rocket::data::Outcome;
use rocket::data::Data;

impl FromDataSimple for People {
    type Error = ();
    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        let contents = String::from("hzjsea");
        let age = 20;

        let p1 = People::new(contents,age);
        Outcome::Success(p1)
    }
}
 

#[get("/people", data = "<people>")]
fn person(people: People) -> String{
    format!("{}:{}",people.name,people.age)
}

// form data

use rocket::request::Form;
use rocket::request::LenientForm;
#[derive(FromForm)]
struct Task {
    #[form(field = "type")]
    complete: bool,
    description: String,
}

// https://stackoverflow.com/questions/45314187/parsing-http-multipart-post-using-a-struct-in-rocket
// 不支持multipart/form-data
// application/x-www-form-urlencoded
#[post("/todo", data = "<task>")]
fn task(task: Form<Task>) -> String{ 
    format!("{}{}",task.complete, task.description)
}


// -----------------------------------------------------------------------------------------------------------------------------
// authon Action
// Post Request
// http://host/auth/
// {
//     "action": "check_auth",
//     "source_addr": String,
//      "data": {
//          "status": u8,
//          "authenticator_source": base64([u8;16])
//       }
// }

use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json as jjson;
use md5::{Md5,Digest};
#[derive(Serialize,Deserialize,Debug)]
struct CheckBodyData{
    status: u8,
    authenticator_source: [u8;16]
}
#[derive(Serialize,Deserialize,Debug)]
struct CheckBody{
    action: String,
    source_addr: String,
    data:CheckBodyData 
}

// 

#[post("/host/auth",format = "application/json", data = "<checkbody>")]
fn auth_data(checkbody:jjson<CheckBody>) -> String{
    let mut body= Vec::new();
    body.push(checkbody.data.status);
    body.extend_from_slice(&checkbody.data.authenticator_source);

    // seearch source_addr  == 888888
    let sharesecret = "888888";
    body.extend_from_slice(&sharesecret.as_bytes());
    // println!("{:?}",body);
    
    let mut hasher = Md5::new();
    hasher.update(&body);
    // println!("{:?}",hasher.finalize());
    let result = hasher.finalize().as_slice().iter().map(|value| value.to_string()).collect::<String>();
    // let result = hasher.finalize()
    format!("{}",result)
}

use diesel::mysql::MysqlConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::request::{FromRequest};
use diesel::Queryable;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

fn connect_mysql() -> Pool{
    let db_url = "mysql://hzj:upyun123@10.0.6.55:3306/test"; 
    let manage = ConnectionManager::<MysqlConnection>::new(db_url);
    r2d2::Pool::new(manage).expect("err mysql connect ")
}

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Conn, ()> {
        let pool = request.guard::<rocket::State<Pool>>()?;
        match pool.get() {
            Ok(conn) => rocket::request::Outcome::Success(Conn(conn)),
            Err(_) => rocket::request::Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = MysqlConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Queryable)]
struct UserTest {
    id: i32,
    name: String,
    age:i32,
}


impl User {
    fn get_all_users(conn: &MysqlConnection) -> Vec<User> {
        
    }
}





fn main() {
    // rocket::ignite().mount("/", routes![pong]).launch();
    let rocket  = rocket::ignite()
        .mount("/",routes![pong,index,echo,auth_data])
        .mount("/api/func", routes![user_echo])
        .mount("/dynamic_path/", routes![mult_path,echo_name])
        // .mount("/file",routes![create_file]);
        .mount("/api/userid/", routes![user,user_int,user_str])
        .mount("/api/query/", routes![hello_quert_map,item_get])
        .mount("/api/auth/", routes![auth])
        .mount("/api/", routes![print_cookie])
        .mount("/api", routes![
            get_id,
            get_str,
            just_fail,
            person,
            task])
        ;
    rocket.launch();
}