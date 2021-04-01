use diesel::{self, associations::HasTable};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::schema;

use super::schema::*;
use super::schema::users::dsl::users as all_users;

#[derive(Queryable,Serialize,Deserialize,Debug)]
pub struct User {
    pub id: i32,
    pub source_addr: String,
    pub shared_secret: String,
}


impl User{
    pub fn get_all_users(conn: &MysqlConnection) -> Vec<User>{
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error")
    }
    pub fn check_auth(source_addr: &str,conn: &MysqlConnection) -> Vec<User>{
        all_users
            .filter(schema::users::Source_Addr.eq_all(source_addr))
            .load::<User>(conn)
            .expect("error")
    }
}
