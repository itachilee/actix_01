use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize,sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub lastmodified: Option<NaiveDateTime>,
}