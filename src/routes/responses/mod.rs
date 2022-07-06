use rocket::response::{self, Response, Responder, content, status};
use rocket::serde::{json::{json, Json}};
use jsonwebtoken::errors::{Error as JWTError, ErrorKind};

use std::error::Error;

use crate::helpers::encrypt::AuthToken;
/*
#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct JsonWebTokenRes {
    pub inner: String
}*/

