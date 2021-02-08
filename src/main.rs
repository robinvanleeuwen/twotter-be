#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]

mod errors;
mod models;
mod schema;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_json;
extern crate custom_error;
extern crate rocket_cors;

use crate::models::{NewTwoot, Twoot, AccountPostData};
use diesel::prelude::*;
use errors::TwotterError;
use models::{TwootPostData, Account};
use rocket_contrib::{json::Json, databases};
use serde_json::Value;
use std::collections::HashMap;

#[database("twotter")]
struct TwotterDBConn(databases::diesel::PgConnection);

#[get("/user/<username>")]
fn get_user_by_username(conn: TwotterDBConn, username: String) -> String {
    match get_user_from_database(&*conn, &username) {
        Ok(x) => json!(x).to_string(),
        Err(e) => format!("{}", e).to_string(),
    }
}

#[post("/twoot/add", format = "application/json", data = "<input>")]
fn add_twoot(conn: TwotterDBConn, input: Json<TwootPostData>) -> String {
    match insert_twoot_in_database(&*conn, &input.username, &input.content) {
        Ok(x) => json!(x).to_string(),
        Err(e) => format!("{}", e).to_string(),
    }
}


#[post("/user/<username>", format = "application/json", data = "<input>")]
fn update_user(conn: TwotterDBConn, username: String, input: Json<AccountPostData>) -> String {
    match update_user_in_database(&*conn, &input.username, &input.email, &input.first_name, &input.last_name) {
        Ok(x) => json!(x).to_string(),
        Err(e) => format!("{}", e).to_string(),
    }
}

fn update_user_in_database(conn: &PgConnection,
                           v_username: &String,
                           v_email: &String,
                           v_first_name: &String,
                           v_last_name: &String) -> Result<Value, TwotterError>{
    use crate::schema::account::dsl::*;

    let result = diesel::update(
        account.filter(username.eq(v_username))
    ).set((
        email.eq(v_email),
        first_name.eq(v_first_name),
        last_name.eq(v_last_name)
    )).execute(&*conn);

    match result {
        Ok(_x) => Ok(json!({
            "username": v_username,
            "email": v_email,
            "first_name": v_first_name,
            "last_name": v_last_name,
        })),
        Err(_e) => Err(
            TwotterError::DBUpdateError { table: "tweet".to_string() }
        )
    }
}

#[get("/twoot/get/byuser/<username>")]
fn get_twoots_by_username(conn: TwotterDBConn, username: String) -> String {

    let _twoots_per_user: (String, HashMap<i32, String>) = match get_twoots_from_database(&*conn, &username){
        Ok(x) => {
          x
        },
        Err(e) => return format!("{}", e).to_string(),
    };
    json!(_twoots_per_user).to_string()

}

#[get("/twoot/get/byuuid/<twoot_uuid>")]
fn get_twoot_by_uuid(conn: TwotterDBConn, twoot_uuid: String) -> String {

    let result = match db_get_twoot_by_uuid(&*conn, twoot_uuid) {
        Ok(x) => x,
        Err(e) => return format!("{}", e).to_string(),
    };
    json!(result).to_string()

}

fn db_get_twoot_by_uuid(conn: &PgConnection, twoot_uuid: String) -> Result<Vec<Twoot>, TwotterError> {
    use crate::schema::twoot::dsl::*;

    let result = twoot.filter(uuid.eq(twoot_uuid)).load::<Twoot>(conn);

    return match result {
        Ok(x) => Ok(x),
        Err(_e) => Err(TwotterError::RecordNotFound { table: "twoot".to_string() })
    };

}

fn get_twoots_from_database(conn: &PgConnection, user_name: &String) -> Result<(String, HashMap<i32, String>), TwotterError>{

    use crate::schema::twoot::dsl::*;
    use crate::schema::account::dsl::*;

    let result: std::result::Result<Vec<((i32,i32,String, String), (i32, String, String, String, String, bool))>, diesel::result::Error> = twoot.inner_join(account).filter(username.eq(user_name)).load(conn);

    let mut twoots: HashMap<i32, String> = HashMap::new();

    match result {
        Ok(x) => {
            for t in x {
                twoots.insert(t.0.0.clone(), t.0.2.clone().to_string());
            }
            Ok((user_name.to_string(), twoots))
        }
        Err(_) => Err(TwotterError::RecordNotFound { table: "Could not twoots for user.".to_string() })
    }
}

fn insert_twoot_in_database(
    conn: &PgConnection,
    tw_username: &String,
    tw_content: &String,
) -> Result<Value, TwotterError> {
    use crate::schema::twoot::dsl::*;

    if tw_content.len() > 144 {
        return Err(TwotterError::TwootMaxCharExceeded { numchar: tw_content.len() as i32 })
    }

    let user = match get_user_from_database(conn, &tw_username) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    let new_twoot: NewTwoot = NewTwoot {
        user_id: serde_json::from_value(user["id"].clone()).unwrap(),
        content: tw_content.clone(),
    };

    let result =
        diesel::insert_into(twoot)
            .values(&new_twoot)
            .execute(&*conn);

    match result {
        Ok(_x) => Ok(json!({
            "username": tw_username,
            "content": tw_content
        }
        )),
        Err(_e) => Err(TwotterError::RecordNotFound {
            table: "tweet".to_string(),
        }),
    }
}



fn get_user_from_database(conn: &PgConnection, un: &String) -> Result<Value, TwotterError> {
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

fn main() {
    let cors_default = rocket_cors::CorsOptions::default();
    let cors = cors_default.to_cors().unwrap();

    rocket::ignite()
        .attach(cors)
        .attach(TwotterDBConn::fairing())
        .mount(
            "/",
            routes![
                get_user_by_username,
                update_user,
                add_twoot,
                get_twoots_by_username,
                get_twoot_by_uuid,
            ],
        )
        .launch();
}
