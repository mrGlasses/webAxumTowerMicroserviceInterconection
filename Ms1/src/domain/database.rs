use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub uid: i32,
    pub name: String,
}