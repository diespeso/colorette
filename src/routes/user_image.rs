use rocket::serde::{json::Json, json::json, Serialize, Deserialize};
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::response::Responder;

use uuid::Uuid;

use crate::models::user_image::*;
use crate::helpers::{StdResult, StdError};
use crate::helpers::encrypt;
use crate::errors;
use crate::controllers::user_image;

/// In the future use a guard for the cookie maybe?
/// TODO: refactor this function to make it shorter and cleaner
#[post("/user_images", data="<image_form>")] //change result err to custom error later
pub async fn upload_user_image(mut image_form: Form<UserImageForm<'_>>, 
    jar: &CookieJar<'_>) -> Result<Json<UserImage>, errors::ImageUploadError> {
    //println!("{:?}", jar.get_pending("jwt").unwrap().value());
    println!("{:?}", image_form);
    let mut flag = false;
    let mut user_folder = "".to_string();
    //jwt check, probs make a function for this
    if let Some(token) = jar.get_pending("jwt") { //user auth cookie present
        if let Ok(auth_token_data) = encrypt::verify_token(token.value(), "SECRETO") {
            user_folder = auth_token_data.claims.username;
            flag = true;
        } else {
            return Err(errors::ImageUploadError::auth_error(
                errors::AuthError::unreadable(token.value().to_string())
            ));
        }
        
    } else { // user auth cookie missing
        return Err(errors::ImageUploadError::auth_error(
            errors::AuthError::no_cookie("jwt")
        ))
    }
    
    if flag {
        println!("saving...{}", user_folder);

        let uuid = Uuid::new_v4().to_string();
        let name = image_form.name.clone();
        image_form.file
            .persist_to(format!("./user_images/{}/{}.jpg", user_folder, uuid))
            .await
            .expect("failed to save image in filesystem"); //sould return imageuploaderror

        //database registration
        match user_image::create(&UserImage::new( //user folder equals user email equals the id of the user (not the actual id, but the one im using for this)
            uuid, user_folder, name
        )) {
            Ok(user_image_data) => {
                Ok(Json::from(user_image_data))
            },
            Err(error) => {
                Err(errors::ImageUploadError::internal_error(error.to_string()))
            }
        }
    } else {
        unreachable!() //should be the case
    }
    
/*
    Ok(
        Json::from(UserImage::new("".to_string(), "".to_string(), "".to_string()))
    )*/
}