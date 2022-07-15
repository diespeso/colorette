use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserImage {
    pub uuid: String,
    pub user_id: String,
    pub name: String,
} //depends on user table

impl UserImage {
    pub fn new(uuid: String, user_id: String, name: String) -> Self {
        Self{uuid, user_id, name}
    }

    pub fn get_schema() -> &'static str {
        r#"
            CREATE TABLE IF NOT EXISTS user_image(
                uuid VARCHAR(40) not null primary key,
                user_id VARCHAR(50) not null,
                name text,
                FOREIGN KEY (user_id)
                    REFERENCES user(email)
            );
        "#
    }
}