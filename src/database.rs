use std::collections::HashMap;

use crate::models::User;

pub struct Database {
    pub users: HashMap<String, User>,
    pub token_counter: u32,
}

impl Database {
    pub fn new() -> Database {
        Database {
            users: HashMap::new(),
            token_counter: 0,
        }
    }
}
