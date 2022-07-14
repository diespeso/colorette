#[macro_use] extern crate rocket;

use std::{io, path::{Path, PathBuf}};

use rocket::response::{self, Response, Responder, content, status};
use rocket::serde::{json::Json, json::json, Serialize, Deserialize};
use rocket::{fs::NamedFile, response::{Redirect}};
use rocket::http::{Cookie, CookieJar};


use helpers::encrypt::{sign_token, verify_token, AuthToken, self};
use helpers::StdError;

mod controllers;
mod models;
mod helpers;
mod database;
mod routes;
mod errors;
mod engine;


#[get("/")]
fn index(jar: &CookieJar<'_>) -> String {
    /*let jwt = jar.get_pending("jwt");
    println!("GET / UWU");
    verify_token(jwt.unwrap().value(), "secreto");
    "ok".to_string()*/
    "ok".to_string()
    /*if let Some(j) = jwt{
        return format!("Bienvenido, {}", j)
    } else {
        return format!("Hola, desconocido")
    }*/
}

#[post("/auth")]
fn auth(jar: &CookieJar<'_>) -> Result<Json<AuthToken>, errors::AuthError> {
    let pending = jar.get_pending("jwt");
    /*if pending.is_none() {
        return Err(errors::AuthError::no_cookie("jwt"))
    }
    let jwt = pending.unwrap();*/
    let jwt;
    match pending {
        Some(cookie) => {
            jwt = cookie;
        },
        None => {
            return Err(errors::AuthError::no_cookie("jwt"));
        }
    }
    println!("{:?}", jwt);
    unimplemented!()
    //verify_token(, secret)
}

#[get("/front/<file..>")]
async fn build_dir(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("build/").join(file)).await
}
/*
#[post("/user", data="<user>")]
fn create_user(user: Json<models::User>) -> String {
    let _user = user.clone().into_inner();
    controllers::user::create(_user);
    format!("Usuario creado: {:?}", user)
}
*/

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenResponse {
    pub jwt: String
}

#[post("/user", data="<user>")]
fn create_user(user: Json<models::User>) -> Result<Json<TokenResponse>, String> {
    //should obscure error
    match controllers::user::create(user.into_inner()) {
        Ok(data) => {
            let token;
            match encrypt::sign_token(json!({"email": data.email}), "SECRETO") {
                Ok(t) => {
                    token = t;
                },
                Err(e) => {
                    return Err(e.to_string());
                }
            }
            let res = Json::from(TokenResponse{jwt: token});
            println!("{:?}", res);
            return Ok(res);
        },
        Err(err) => {
            println!("{:?}", err);
            return Err(err.to_string());
        }
    }
}

#[put("/user/<id>", data="<user>")]
fn update_user(id: i32, user: Json<models::User>) -> String {
    let new = user.into_inner();
    let old = controllers::user::update(new.clone())
        .expect("Failed to update user");
    format!("Updated user from {:?} to {:?}", old, new)
}

#[get("/user/<id>")]
fn get_user(id: i32) -> Json<models::User> {
    let usr = controllers::user::read(id)
        .expect("Failed to get user");
    Json(usr)
}

#[get("/user")]
fn get_user_list() -> Json<Vec<models::User>> {
    let users = controllers::user::readList()
        .expect("Failed to get user list");
    Json(users)
}

#[delete("/user/<id>")]
fn delete_user(id: i32) -> String {
    match controllers::user::delete(id) {
        Ok(usr) => format!("User deleted: {:?}", usr),
        Err(err) => format!("Something went wrong: {:?}", err)
    }
}

#[launch]
fn rocket() -> _ {
    database::create_schema();
    rocket::build()
    .mount("/", routes![index, build_dir])
    .mount("/api", routes![auth, create_user, get_user, update_user, delete_user, get_user_list])
    .mount("/api", routes![routes::session::create_session])
}

/*
use std::env;
use crate::engine::PaletteExtractor;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 4 {
        println!("{:?}", "args should be: <filename> <k> <iterations>");
        return
    }
    let palette = PaletteExtractor::from_image_file(args[1].clone())
        .with_k(args[2].parse::<u32>().expect("wrong k value in command line"))
        .extract(args[3].parse::<u32>().expect("wrong iteration value in command line"));
    println!("palette: {:?}", palette);
}*/


