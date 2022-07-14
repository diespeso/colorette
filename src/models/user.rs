use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub pass: String,
}

impl User {
    pub fn new(id: i32, email: String, pass: String) -> Self {
        User {
            id: Some(id),
            email,
            pass
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserSearch {
 pub id: Option<i32>,
 pub email: Option<String>,
 pub pass: Option<String>   
}
