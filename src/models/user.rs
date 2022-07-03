use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub pass: String,
}

impl User {
    pub fn new(id: i32, username: String, pass: String) -> Self {
        User {
            id: Some(id),
            username,
            pass
        }
    }
}
