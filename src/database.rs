use mysql::*;
use mysql::prelude::*;

const CONNECTION_STRING: &str = "mysql://root@localhost:3306/test1";

use crate::helpers::{StdError, StdResult};

pub fn get_conn() -> StdResult<PooledConn, Box<StdError>> {
    Ok(Pool::new(CONNECTION_STRING)?.get_conn()?)
}