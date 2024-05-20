use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub id: Thing,
    pub username: String,
    pub password: String,
    pub email: String,

    pub nickname: String,
    pub favicon: String,
    pub introduction: Option<String>,
    pub website: Option<String>,
    pub organization: Option<String>,

    pub privilege: i32,
    pub status: i32,

    pub last_login: Datetime,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub id: Thing,
    pub account: Thing,
    pub token: String,
    pub expired_at: Datetime,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
