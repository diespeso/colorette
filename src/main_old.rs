#[macro_use] extern crate rocket;

use mysql::*;
use mysql::prelude::*;

use rocket::http::{Status, ContentType};
use rocket::response::{self, Response, Responder, content, status};
use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct User {
    id: Option<i32>,
    username: String,
    pass: String
}

#[derive(Debug)]
struct BDNotFoundError{
    table_name: String,
    params: Vec<String>
}

impl BDNotFoundError {
    fn new(table_name: String, params: Vec<String>) -> Self {
        BDNotFoundError{table_name, params}
    }

    fn from_str(table_name: &str, params: Vec<&str>) -> Self {
        let mut nparams = Vec::new();
        params.iter().map(|param| {
            nparams.push(String::from(param.clone()))
        });
        Self::new(String::from(table_name), nparams)
    }
}

impl std::fmt::Display for BDNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Row with params [{:?}] not found in {}", self.params, self.table_name)
    }
}

impl std::error::Error for BDNotFoundError {}

impl User {
    fn new(id: i32, username: String, pass: String) -> Self {
        User{
            id: Some(id),
            username,
            pass}
    }

    fn from_db(id: i32) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let mut conn = getConn()?;
        let res = conn.query_map(format!("SELECT * FROM USER WHERE ID = {}", id),
        |(id, username, pass)|{
            User::new(id, username, pass)
        })?;
        if res.len() == 0 {
            return Err(
                Box::new(BDNotFoundError::new("User".to_owned(), vec![format!("id: {}", id)]))
            )
        }
        Ok(res[0].clone())        
    }
}

#[get("/")]
fn index() -> String {
    String::from("Hola, mundo!")
}

#[derive(Responder)]
#[response(status=200, content_type="json")]
struct GetUsers {
    users: content::RawJson<String>
}

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    let mut conn = getConn().unwrap();
    let res = conn.query_map(
        "SELECT * FROM  user",
        |(id, username, pass)| {
            User {id, username, pass}
        }
    ).expect("error al consultar bd");
    Json::from(res)
}

#[get("/user/<id>")]
fn get_user(id: i32) -> Option<Json<User>> {
    let user = User::from_db(id);
    match user {
        Ok(usr) => Some(Json::from(usr)),
        Err(err) => {
            println!("{}", err);
            None
        }
    }
    
}

#[post("/user", format="application/json", data="<user>")]
fn new_user(user: Json<User>) -> Json<String> {
    let mut conn = getConn().expect("fallo al crear conexion");
    insertIntoUser(&mut conn, user.into_inner()).expect("failed to insert user");
    println!("served post!");
    Json::from(String::from("Todo bien!"))
}

fn getConn() -> std::result::Result<PooledConn, Box<dyn std::error::Error>> {
    let url = "mysql://root@localhost:3306/test1";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    Ok(conn)
}

fn seedTables(mut conn: PooledConn) -> std::result::Result<PooledConn, Box<dyn std::error::Error>> {
    conn.query_drop(
        r"CREATE TABLE if not exists USER(
            id int not null auto_increment primary key,
            username text not null,
            pass text not null   
        )"
    )?;
    Ok(conn)
}

fn insertIntoUser(conn: &mut PooledConn, user: User) -> std::result::Result<(), Box<dyn std::error::Error>> {
    conn.exec_drop(
        r"INSERT INTO USER(username, pass)
        VALUES(:username, :pass)",
        params!{
            "username" => user.username,
            "pass" => user.pass}
    )?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    let mut conn = getConn().expect("Fallo al crear conexion");
    conn = seedTables(conn).expect("Fallo al seedear tablas");
    insertIntoUser(&mut conn, User::new(-1, String::from("Juanito"), String::from("ok"))).expect("Fallo al insertar Usuario");
    rocket::build().mount("/", routes![index])
        .mount("/api", routes![get_users, new_user, get_user])
}
