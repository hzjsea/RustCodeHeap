use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable)]
struct UserTest{
    id: i32,
    name: String,
    age: i32
}






