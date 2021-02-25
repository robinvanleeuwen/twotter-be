use crate::errors::TwotterError;
use crate::models::{Account, AccountPostData, NewAccount};
use crate::TwotterDBConn;
use core::result::Result;
use core::result::Result::{Err, Ok};
use diesel::pg::PgConnection;
use rocket_contrib::json::Json;
use serde_json::value::Value;

use diesel::prelude::*;

#[post("/adduser", format = "application/json", data = "<input>")]
pub(crate) fn create_user(conn: TwotterDBConn, input: Json<NewAccount>) -> String {
    let x = match db_insert_new_user(
        &*conn,
        &input.username,
        &input.first_name,
        &input.last_name,
        &input.email,
        &input.is_admin,
    ) {
        Ok(x) => json!(x).to_string(),
        Err(e) => format!("{}", e).to_string(),
    };
    x
}

#[get("/user/<username>")]
pub(crate) fn get_user_by_username(conn: TwotterDBConn, username: String) -> String {
    match db_get_user_by_username(&*conn, &username) {
        Ok(x) => json!(x).to_string(),
        Err(e) => format!("{}", e).to_string(),
    }
}

#[put("/user", format = "application/json", data = "<input>")]
pub(crate) fn update_user(
    conn: TwotterDBConn,
    input: Json<AccountPostData>,
) -> String {
    match db_update_user(
        &*conn,
        &input.username,
        &input.email,
        &input.first_name,
        &input.last_name,
    ) {
        Ok(x) => json!(x).to_string(),
        Err(e) => format!("{}", e).to_string(),
    }
}

#[get("/users")]
pub(crate) fn get_all_users(conn: TwotterDBConn) -> String {
    match db_get_all_users(&*conn) {
        Ok(x) => json!(x).to_string(),
        Err(e) => format!("{}", e).to_string(),
    }
}

fn db_get_all_users(conn: &PgConnection) -> Result<Vec<Account>, TwotterError> {
    use crate::schema::account::dsl::*;

    let result = account.limit(15).load::<Account>(&*conn);

    match result {
        Ok(x) => Ok(x),
        Err(_e) => Err(TwotterError::RecordNotFound {
            table: "account".to_string(),
        }),
    }
}

fn db_insert_new_user(
    conn: &PgConnection,
    v_username: &String,
    v_first_name: &String,
    v_last_name: &String,
    v_email: &String,
    v_is_admin: &bool,
) -> Result<Value, TwotterError> {
    use crate::schema::account::dsl::*;

    let new_account: NewAccount = NewAccount {
        username: v_username.to_string(),
        first_name: v_first_name.to_string(),
        last_name: v_last_name.to_string(),
        email: v_email.to_string(),
        is_admin: *v_is_admin,
    };

    let result = diesel::insert_into(account)
        .values(&new_account)
        .execute(&*conn);

    match result {
        Ok(_x) => Ok(json!({
            "username": v_username,
            "email": v_email,
            "first_name": v_first_name,
            "last_name": v_last_name,
        })),
        Err(_e) => Err(TwotterError::DBInsertError {
            table: "tweet".to_string(),
        }),
    }
}

fn db_update_user(
    conn: &PgConnection,
    v_username: &String,
    v_email: &String,
    v_first_name: &String,
    v_last_name: &String,
) -> Result<Value, TwotterError> {
    use crate::schema::account::dsl::*;

    let result = diesel::update(account.filter(username.eq(v_username)))
        .set((
            email.eq(v_email),
            first_name.eq(v_first_name),
            last_name.eq(v_last_name),
        ))
        .execute(&*conn);

    match result {
        Ok(_x) => Ok(json!({
            "username": v_username,
            "email": v_email,
            "first_name": v_first_name,
            "last_name": v_last_name,
        })),
        Err(_e) => Err(TwotterError::DBUpdateError {
            table: "tweet".to_string(),
        }),
    }
}

pub fn db_get_user_by_username(conn: &PgConnection, un: &String) -> Result<Value, TwotterError> {
    use crate::schema::account::dsl::*;


    let result = account
        .filter(username.eq(&un))
        .load::<Account>(&*conn)
        .expect("Error loading posts");

    let x = serde_json::json!(result);

    match x.as_array().unwrap().len() {
        1 => Ok(json!(x[0])),
        0 => Err(TwotterError::RecordNotFound {
            table: "user".to_string(),
        }),
        _ => Err(TwotterError::MultipleResults {
            table: "user".to_string(),
        }),
    }
}
