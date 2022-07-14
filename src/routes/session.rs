use crate::TokenResponse;
use rocket::serde::{json::Json, json::json, Serialize, Deserialize};

use crate::controllers::user;
use crate::models;
use crate::helpers::encrypt;


#[post("/session", data="<user>")]
pub fn create_session(user: Json<models::User>) -> Result<Json<TokenResponse>, String>{
    let created = user::create(user.into_inner());
    match created {
        Ok(user_data) => {
            let token;
            match encrypt::sign_token(json!({"email": user_data.email}), "SECRETO") {
                Ok(t) => {
                    token = t;
                },
                Err(e) => {
                    return Err(e.to_string());
                }
            }
            let res = Json::from(TokenResponse{jwt: token});
            return Ok(res);
        },
        Err(err) => {
            return Err(err.to_string());
        }
    }
}