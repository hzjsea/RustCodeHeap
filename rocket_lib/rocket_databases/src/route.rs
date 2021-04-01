use std::vec;

use super::db::Conn as DbConn;
use rocket_contrib::json::{self, Json};
use super::models::{User};
use serde_json::Value;
use md5::{Digest, Md5};
use base64;

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



#[derive(Deserialize,Serialize,Debug)]
pub struct CheckBodyData{
    authenticator_source: String,
    timestamp: i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct CheckBody{
    action: String,
    source_addr: String,
    data: CheckBodyData
}

#[derive(Serialize,Deserialize,Debug)]
pub struct BodyResponse{
    status: u8,
    authenticator_ismg: String,
    msg: String,
}
#[derive(Serialize, Deserialize,Debug)]
pub struct CheckBodyRespose{
    result: bool,
    response: BodyResponse
}

impl CheckBodyRespose {
    fn new(result:bool, status: u8, authenticator_ismg: String, msg:String ) -> Self{
        let br = BodyResponse{
            status:status,
            authenticator_ismg:authenticator_ismg,
            msg:msg
        };
        CheckBodyRespose{
            result:result,
            response: br
        }
    }
}

#[post("/host/auth",format = "application/json", data = "<checkbody>")]
pub fn auth_data(checkbody:Json<CheckBody>,conn:DbConn) -> String{
    let res = User::check_auth(&checkbody.source_addr, &conn);
    let mut shared_secret = "";
    match res.first() {
        Some(value) => {
            shared_secret = &value.shared_secret
        }
        _ => {
            println!("not found")
        }
    }
    let salt = vec![0;9];
    let mut gen_authenticator_surce = Vec::new();
    let source = base64::decode(&checkbody.source_addr).unwrap();
    gen_authenticator_surce.extend_from_slice(source.as_slice());
    gen_authenticator_surce.extend_from_slice(&salt); 
    gen_authenticator_surce.extend_from_slice(&shared_secret.as_bytes());
    gen_authenticator_surce.extend_from_slice(&checkbody.data.timestamp.to_string().as_bytes());
    let mut hasher = Md5::new();
    // let xx= "OTAwMDAx000000000888888331155752";
    hasher.update(&gen_authenticator_surce);

    let result = hasher.finalize();
    let mut authenticator_ismg: [u8;16] = [0;16];
    authenticator_ismg[..].copy_from_slice(&result.as_slice()[..]);
    let res = base64::encode(&authenticator_ismg);
    let cc = CheckBodyRespose::new(true,0,res,"success".to_owned());
    let value = serde_json::to_value(cc).unwrap();
    value.to_string()
}

