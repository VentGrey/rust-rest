use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Client {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub paid: i32,
    pub age: i32
}
