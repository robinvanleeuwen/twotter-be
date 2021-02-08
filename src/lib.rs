use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;

use r2d2;
use r2d2_postgres::postgres::NoTls;
use rocket_contrib::database;

#[database("twotter")]
#[derive(Debug)]
pub struct TwotterDB(diesel::PgConnection);

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_connection_pool() -> r2d2::Pool<r2d2_postgres::PostgresConnectionManager<NoTls>> {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL missing from environment variables");
    let manager =
        r2d2_postgres::PostgresConnectionManager::new(database_url.parse().unwrap(), NoTls);
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();
    pool
}
