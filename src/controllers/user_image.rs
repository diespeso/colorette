
use crate::models::{self, user_image::UserImage};
use crate::helpers::{StdError, StdResult};

/// Creates a user_image in database
pub fn create(userImage: &UserImage) -> StdResult<UserImage, Box<StdError>> {
    unimplemented!()
}

/// stores the image of userImage in the filesystem
pub fn store(userImage: &UserImage) -> StdResult<UserImage, Box<StdError>> {
    unimplemented!()
}