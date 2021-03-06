use mysql::*;
use mysql::prelude::*;

use crate::models::*;

const CONNECTION_STRING: &str = "mysql://root@localhost:3306/test1";

use crate::helpers::{StdError, StdResult};

pub fn get_conn() -> StdResult<PooledConn, Box<StdError>> {
    Ok(Pool::new(CONNECTION_STRING)?.get_conn()?)
}

pub fn create_schema() -> StdResult<(), Box<StdError>> {
    let mut conn = get_conn()?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS USER(
            id int not null auto_increment primary key,
            email VARCHAR(50) unique not null,
            pass text not null
        )"
    )?;
    conn.query_drop(user_image::UserImage::get_schema())?;
    Ok(())
}