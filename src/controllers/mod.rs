pub mod user;

use std::collections::HashMap;

#[derive(Debug)]
pub struct BDNotFoundError {
    table: String,
    params: HashMap<String, String>
}

impl BDNotFoundError {
    pub fn new(table: String, params: HashMap<String, String>) -> Self {
        BDNotFoundError{table, params}
    }

    pub fn from_table_id(table: String, id: i32) -> Self{
        BDNotFoundError { table, params: HashMap::from([("id".to_string(), id.to_string())]) }
    }
    
    pub fn from_table(table: String) -> Self {
        BDNotFoundError {table, params: HashMap::new() }
    }
}

impl std::fmt::Display for BDNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Row with params {:?} not found in table {}", self.params, self.table)
    }
}

impl std::error::Error for BDNotFoundError{}