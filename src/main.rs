#[macro_use] extern crate rocket;

use std::{io, path::{Path, PathBuf}};

use rocket::response::{self, Response, Responder, content, status};
use rocket::serde::{json::Json, json::json};
use rocket::{fs::NamedFile, response::{Redirect}};
use rocket::http::{Cookie, CookieJar};

use routes::responses::JsonWebTokenRes;
use helpers::encrypt::{sign_token, verify_token, AuthToken};

mod controllers;
mod models;
mod helpers;
mod database;
mod routes;


#[get("/")]
fn index(jar: &CookieJar<'_>) -> String {
    let jwt = jar.get_pending("jwt");
    println!("GET / UWU");
    verify_token(jwt.unwrap().value(), "secreto");
    "ok".to_string()
    /*if let Some(j) = jwt{
        return format!("Bienvenido, {}", j)
    } else {
        return format!("Hola, desconocido")
    }*/
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
#[post("/user", data="<user>")]
fn create_user(user: Json<models::User>) -> JsonWebTokenRes {
    JsonWebTokenRes{
        inner: sign_token(
            AuthToken::new(0, "0".to_string()),
            "secreto"
        ).unwrap()
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
    rocket::build()
    .mount("/", routes![index, build_dir])
    .mount("/api", routes![create_user, get_user, update_user, delete_user, get_user_list])

}
