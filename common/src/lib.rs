use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub password: String,
}
