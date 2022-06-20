use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection(db_url: Option<&str>) -> MysqlConnection {
    dotenv().ok();

    let database_url;
    if db_url.is_none() {
        database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be provided when not passing a database_name");
    } else {
        database_url = String::from(db_url.unwrap());
    }

    println!("Connecting to: {}", database_url);
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}