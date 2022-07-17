use std::collections::HashMap;

use crate::models::{self, user_image::UserImage};
use crate::helpers::{StdError, StdResult};
use crate::database::get_conn;
use super::BDNotFoundError;

use mysql::*;
use mysql::prelude::*;

/// Creates a user_image in database
pub fn create(user_image: &UserImage) -> StdResult<UserImage, Box<StdError>> {
    let mut conn = get_conn()?;
    //create
    conn.exec_drop(
        r"INSERT INTO user_image(uuid, user_id, name)
            VALUES(:uuid, :user_id, :name)",
            params!{
                "uuid" => user_image.uuid.clone(),
                "user_id" => user_image.user_id.clone(),
                "name" => user_image.name.clone()
            }
    )?;
    //retrieve and send back
    let row = conn.query_map(format!(r#"SELECT * FROM user_image WHERE uuid = "{}""#, user_image.uuid.clone()),
|(uuid, user_id, name)| { //make userimage out of row
    UserImage::new(uuid, user_id, name)
})?; 

    if row.len() == 0 { //not found
        Err(Box::new(
            BDNotFoundError::new("user_images".to_owned(), HashMap::new())
        ))
    } else {
        Ok(row[0].clone())
    }
}

/// stores the image of userImage in the filesystem
pub fn store(userImage: &UserImage) -> StdResult<UserImage, Box<StdError>> {
    unimplemented!()
}