use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection(database_name: Option<String>) -> MysqlConnection {
    dotenv().ok();

    let database_url;
    if database_name.is_none() {
        database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be provided when not passing a database_name");
    } else {
        database_url = format!("db/{}", database_name.unwrap());
    }

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}