use diesel::{self, associations::HasTable};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use super::schema::users;
use super::schema::users::dsl::users as all_users;

#[derive(Queryable,Serialize,Deserialize,Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub sharesecret: String,
    pub authenticator_source: String,
}


impl User{
    pub fn get_all_users(conn: &MysqlConnection) -> Vec<User>{
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error")
    }
    pub fn check_auth(authenticator_source: &str,conn: &MysqlConnection) -> Vec<User>{
        all_users
            .filter(users::authenticator_source.eq_all(authenticator_source))
            .load::<User>(conn)
            .expect("error")
        
    }
}
