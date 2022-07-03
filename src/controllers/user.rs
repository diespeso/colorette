use std::collections::HashMap;
use std::hash::Hash;

use mysql::*;
use mysql::prelude::*;

use crate::helpers::{StdResult, StdError};

use crate::models::user::User;
use super::BDNotFoundError;
use crate::database::get_conn;

pub fn create(user: User) -> StdResult<User, Box<StdError>> {
    let mut conn = get_conn()?;
    conn.exec_drop(
        r"INSERT INTO USER(username, pass)
            VALUES(:username, :pass)",
            params!{
                "username" => user.username.clone(),
                "pass" => user.pass.clone()
            }
    )?;
    Ok(user)
}

pub fn read(id: i32) -> StdResult<User, Box<StdError>> {
    let mut conn = get_conn()?;
    let row = conn.query_map(format!("SELECT * FROM USER WHERE ID =  {}", id),
    |(id, username, pass)| {
        User::new(id, username, pass)
    })?;
    if row.len() == 0 {
        Err(
            Box::new(
                BDNotFoundError::new("User".to_owned(), HashMap::new())
            )
        )
    } else {
        Ok(row[0].clone())
    }
}

pub fn readList() -> StdResult<Vec<User>, Box<StdError>> {
    let mut conn = get_conn()?;
    /*let rows = conn.query_map(format!("SELECT * FROM USER"),
        |(id, username, pass)| {

        }
    })?;*/
    let rows = conn.query_map(format!("SELECT * FROM USER"),
        |(id, username, pass)| {
            User::new(id, username, pass)
        }    
    )?;
    if(rows.len() == 0) {
        Err(
            Box::new(
                BDNotFoundError::from_table("User".to_string())
            )
        )
    } else {
        Ok(rows)
    }
}

pub fn update(user: User) -> StdResult<User, Box<StdError>> {
    let usr = read(user.id.expect("Cant update user without valid id"))?;
    let mut conn = get_conn()?;
    conn.exec_drop(
        r#"UPDATE User SET
            username = :username,
            pass = :pass
            WHERE id = :id
            "#,
        params!{
            "username" => user.username,
            "pass" => user.pass,
            "id" => user.id
        }
    )?;
    Ok(usr)
}

pub fn delete(id: i32) -> StdResult<User, Box<StdError>> {
    let old: User;
    match read(id) { //could be something else, idk
        Ok(o) => old = o,
        Err(_) => return Err(Box::new(
            BDNotFoundError::from_table_id("User".to_string(), id)
        ))
    };
    let mut conn = get_conn()?;
    let res = conn.exec_drop(
        r"
        DELETE FROM User WHERE id = :id",
        params!{"id" => id})?;
    Ok(old)
}