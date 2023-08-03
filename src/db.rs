use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection(db_url: Option<&str>) -> MysqlConnection {
    dotenv().ok();

    let database_url: String;

    let mysql_user: String;
    let mysql_password: String;
    let mysql_host: String;
    let mysql_port: String;
    let mysql_database: String;
    mysql_user = env::var("MYSQL_USER").expect("MYSQL_USER must be provided");
    mysql_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD must be provided");
    mysql_host = env::var("MYSQL_HOST").expect("MYSQL_HOST must be provided");
    mysql_port = env::var("MYSQL_PORT").expect("MYSQL_PORT must be provided");
    mysql_database = env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE must be provided");
    database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        mysql_user, mysql_password, mysql_host, mysql_port, mysql_database
    );

    println!("Connecting to: {}", database_url);
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
