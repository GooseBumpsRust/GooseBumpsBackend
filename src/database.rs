use std::collections::HashMap;

use crate::models::User;

pub struct Database {
    pub users: HashMap<String, User>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            users: HashMap::new(),
        }
    }
}
