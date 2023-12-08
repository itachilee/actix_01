use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub lastmodified: Option<NaiveDateTime>,
}