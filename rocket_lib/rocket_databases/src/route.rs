use super::db::Conn as DbConn;
use diesel::IntoSql;
use rocket_contrib::json::Json;
use super::models::{User};
use serde_json::Value;
use md5::{Md5,Digest};

#[get("/all")]
pub fn get_all(conn: DbConn) -> Json<Value>{
    Json(json!(
        {
            "status": 200,
            "result": User::get_all_users(&conn)
        }
    ))
}


#[get("/")]
pub fn index() -> String{
    format!("{}","http://localhost:9600/users/all")
}

#[derive(Serialize,Deserialize,Debug)]
pub struct CheckBodyData{
    status: u8,
    authenticator_source: [u8;16]
}
#[derive(Serialize,Deserialize,Debug)]
pub struct CheckBody{
    action: String,
    source_addr: String,
    data:CheckBodyData
}

//

#[post("/host/auth",format = "application/json", data = "<checkbody>")]
pub fn auth_data(checkbody:Json<CheckBody>,conn:DbConn) -> String{
    let mut body= Vec::new();
    body.push(checkbody.data.status);
    body.extend_from_slice(&checkbody.data.authenticator_source);

    // search source_addr in  mysql 
    let authenticator_source = &checkbody.data.authenticator_source;
    let authenticator = authenticator_source.iter().map(|value | value.to_string()).collect::<String>();
    let sharesecret = User::check_auth(
        &authenticator,&conn);
    let sharesecret = sharesecret.first();
    match sharesecret {
        Some(value) => {
            println!("{:?}",value)
        }
        _ => println!("error")
    }
    println!("{:?}",&sharesecret);
    // body.extend_from_slice(&sharesecret.as_bytes());
    

    // seearch source_addr  == 888888
    // let sharesecret = "888888";
    // body.extend_from_slice(&sharesecret.as_bytes());
    // println!("{:?}",body);

    let mut hasher = Md5::new();
    hasher.update(&body);
    // println!("{:?}",hasher.finalize());
    let result = hasher.finalize().as_slice().iter().map(|value| value.to_string()).collect::<String>();
    // let result = hasher.finalize()
    format!("{}",result)
}

