use std::collections::HashMap;
use std::hash::Hash;

use mysql::*;
use mysql::prelude::*;

use crate::helpers::encrypt::sha256;

use crate::helpers::{StdResult, StdError};

use crate::models::{self, user::User};
use super::BDNotFoundError;
use crate::database::get_conn;

pub fn create(user: User) -> StdResult<User, Box<StdError>> {
    let mut conn = get_conn()?;
    conn.exec_drop(
        r"INSERT INTO USER(email, pass)
            VALUES(:email, :pass)",
            params!{
                "email" => user.email.clone(),
                "pass" => sha256(user.pass.clone()), //TODO: Reconsiderar si deberia encriptar aqui o al recibir el request
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

/// Looks for a user in database using a UserSearch instance
pub fn search(user: models::user::UserSearch) -> StdResult<User, Box<StdError>> {
    //look for it in db
    let mut conn = get_conn()?;

    let mut query = "SELECT * FROM user".to_string(); //building query
    let mut changed = false;
    if let Some(s_id) = user.id {
        query += format!(r#" where id = "{}""#, s_id).as_ref();
        changed = true;
    }
    if let Some(s_email) = user.email {
        if changed {
            query += format!(r#" and email = "{}""#, s_email).as_ref();
        } else {
            query += format!(r#" where email = "{}""#, s_email).as_ref();
            changed = true;
        }
    }
    //search by pass not valid
    let result = conn.query_map(query, |(id, email, pass)| { //map db user to model user
        User::new(id, email, pass)
    })?;
    if result.len() == 0 { //user not found
        Err(
            Box::new(BDNotFoundError::new("User".to_owned(), HashMap::new()))
        )
    } else {
        Ok(result[0].clone())
    }

    //make the User obj
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
            email = :email,
            pass = :pass
            WHERE id = :id
            "#,
        params!{
            "email" => user.email,
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

#[cfg(test)]
pub mod test {
    use super::search;
    use crate::models::user::{User, UserSearch};
    #[test]
    fn test_search_user() {
        let search_data = UserSearch{id: None, email: Some("test".to_string()), pass: None};
        let res = search(search_data).expect("failed in user search i guess");
        println!("{:?}", res);
    }
}