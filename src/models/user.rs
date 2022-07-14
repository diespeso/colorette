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
/// Represents the search for a user in database
/// the pass parameter is ignored, but kept here for consistency
/// with the User struct
pub struct UserSearch {
 pub id: Option<i32>,
 pub email: Option<String>,
 pub pass: Option<String>   
}

impl UserSearch {
    pub fn from_user_ref(user: &User) -> Self {
        Self {
            id: user.id,
            email: Some(user.email.clone()),
            pass: Some(user.pass.clone())
        }
    }
}