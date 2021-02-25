use super::schema::{account, twoot};
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Queryable, Debug, Deserialize)]
pub(crate) struct Account {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Serialize, Insertable, Deserialize)]
#[table_name = "account"]
pub(crate) struct NewAccount {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize, Queryable)]
pub(crate) struct Twoot {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub uuid: String,
}

#[derive(Deserialize, Insertable, Queryable)]
#[table_name = "twoot"]
pub(crate) struct NewTwoot {
    pub user_id: i32,
    pub content: String,
}

#[derive(Deserialize)]
pub(crate) struct TwootPostData {
    pub username: String,
    pub content: String,
}

#[derive(Deserialize)]
pub(crate) struct AccountPostData {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
pub(crate) struct TwootsPerUsername {
    pub username: String,
    pub twoot_ids: Vec<HashMap<i32, String>>,
}
