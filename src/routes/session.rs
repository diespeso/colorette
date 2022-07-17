use crate::TokenResponse;
use rocket::serde::{json::Json, json::json, Serialize, Deserialize};

use crate::controllers;
use crate::models;
use crate::helpers::{self, encrypt};
use crate::errors;


#[post("/session", data="<user>")]
pub fn create_session(user: Json<models::User>) -> Result<Json<TokenResponse>, errors::SessionError>{
    let user_data = user.clone().into_inner();
    let search_data = models::user::UserSearch::from_user_ref(&user_data);
    let read = controllers::user::search(search_data);
    match read { 
        Ok(result) => { //user found
            //check password
            if !encrypt::verify_sha256(&result.pass, &user_data.pass) {
                return Err(errors::SessionError::auth_error("Wrong password".to_string()))
            }
            //generate jwt
            match encrypt::sign_token(encrypt::AuthToken::new(-1, user_data.email), "SECRETO") {
                Ok(t) => {
                    Ok(Json::from(TokenResponse{jwt: t})) //send generated token
                },
                Err(e) => { //jwt error, special error
                    println!("unhandled: {:?}", e);
                    Err(errors::SessionError::auth_error(e.to_string()))
                }
            } 
        }, Err(err) => {
            //let e = errors::SessionError::not_found(user.email.clone());
            let e = errors::SessionError::not_found(err.to_string());
            println!("{}", e);
            Err(e)
        }
    }
}