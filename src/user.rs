use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Users {
    pub users: Vec<User>,
}
