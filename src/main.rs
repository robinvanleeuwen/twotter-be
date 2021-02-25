#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]

use std::collections::HashMap;

use diesel::prelude::*;
use rocket_contrib::{databases, json::Json};
use serde_json::Value;

use errors::TwotterError;
use models::{TwootPostData};

use crate::models::{NewTwoot, Twoot};
use crate::users::static_rocket_route_info_for_create_user;
use crate::users::static_rocket_route_info_for_get_all_users;
use crate::users::static_rocket_route_info_for_get_user_by_username;
use crate::users::db_get_user_by_username;
use crate::users::static_rocket_route_info_for_update_user;

mod errors;
mod models;
mod schema;
mod users;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;
extern crate custom_error;
extern crate rocket_cors;

#[database("twotter")]
struct TwotterDBConn(databases::diesel::PgConnection);

#[post("/twoot/add", format = "application/json", data = "<input>")]
fn add_twoot(conn: TwotterDBConn, input: Json<TwootPostData>) -> String {
    match insert_twoot_in_database(&*conn, &input.username, &input.content) {
        Ok(x) => json!(x).to_string(),
        Err(e) => format!("{}", e).to_string(),
    }
}

#[get("/twoot/get/byuser/<username>")]
fn get_twoots_by_username(conn: TwotterDBConn, username: String) -> String {
    let _twoots_per_user: (String, HashMap<i32, String>) =
        match db_get_twoots_by_username(&*conn, &username) {
            Ok(x) => x,
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

fn db_get_twoot_by_uuid(
    conn: &PgConnection,
    twoot_uuid: String,
) -> Result<Vec<Twoot>, TwotterError> {
    use crate::schema::twoot::dsl::*;

    let result = twoot.filter(uuid.eq(twoot_uuid)).load::<Twoot>(conn);

    return match result {
        Ok(x) => Ok(x),
        Err(_e) => Err(TwotterError::RecordNotFound {
            table: "twoot".to_string(),
        }),
    };
}

fn db_get_twoots_by_username(
    conn: &PgConnection,
    user_name: &String,
) -> Result<(String, HashMap<i32, String>), TwotterError> {
    use crate::schema::account::dsl::*;
    use crate::schema::twoot::dsl::*;

    let result: std::result::Result<
        Vec<(
            (i32, i32, String, String),
            (i32, String, String, String, String, bool),
        )>,
        diesel::result::Error,
    > = twoot
        .inner_join(account)
        .filter(username.eq(user_name))
        .load(conn);

    let mut twoots: HashMap<i32, String> = HashMap::new();

    match result {
        Ok(x) => {
            for t in x {
                twoots.insert(t.0 .0.clone(), t.0 .2.clone().to_string());
            }
            Ok((user_name.to_string(), twoots))
        }
        Err(_) => Err(TwotterError::RecordNotFound {
            table: "Could not find twoots for user.".to_string(),
        }),
    }
}

fn insert_twoot_in_database(
    conn: &PgConnection,
    tw_username: &String,
    tw_content: &String,
) -> Result<Value, TwotterError> {
    use crate::schema::twoot::dsl::*;

    if tw_content.len() > 144 {
        return Err(TwotterError::TwootMaxCharExceeded {
            numchar: tw_content.len() as i32,
        });
    }

    let user = match db_get_user_by_username(conn, &tw_username) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    let new_twoot: NewTwoot = NewTwoot {
        user_id: serde_json::from_value(user["id"].clone()).unwrap(),
        content: tw_content.clone(),
    };

    let result = diesel::insert_into(twoot)
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
                create_user,
                get_all_users,
                add_twoot,
                get_twoots_by_username,
                get_twoot_by_uuid,
            ],
        )
        .launch();
}
