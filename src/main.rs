#![feature(decl_macro)]
#[macro_use] extern crate rocket;

#[macro_use]
extern crate diesel;
use self::diesel::prelude::*;
// use crate::diesel::QueryDsl;
// use crate::diesel::query_dsl::limit_dsl::LimitDsl;

extern crate dotenv;

use std::process::id;
use rocket::response::content::Json;

pub mod util;
pub mod db;
pub mod schema;
pub mod models;

use crate::util::{get_data_uri_for_avatar, get_version_code_from_string};

use db::*;
use crate::models::Avatar;

// Endpoint Definitions
#[get("/version")]
fn version() -> Json<String> {
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");
    let version_info = format!(r#"
      "name": "{}",
      "version": "{}",
      "version_code": {}
    "#, pkg_name, pkg_version, get_version_code_from_string(pkg_version));
    let mut json_obj = String::from("{");
    json_obj.push_str(version_info.as_str());
    json_obj.push_str("}");
    Json(json_obj)
}

#[get("/avatar/<query_id>")]
fn get_avatar_by_id(query_id: i32) -> Json<String> {
    use crate::schema::avatars::dsl::*;

    let connection = establish_connection();
    let results = avatars
        .filter(id.eq(&query_id))
        .limit(1)
        .load::<Avatar>(&connection)
        .expect("Error loading avatar");

    let json_body = format!(r#"
        "id": {},
        "image": "{}",
        "mimetype": "{}",
        "created": "{}",
        "data_uri": "{}"
    "#, results[0].id, results[0].image, results[0].mimetype, results[0].created, get_data_uri_for_avatar(&results[0]));
    let mut json_obj = String::from("{");
    json_obj.push_str(json_body.as_str());
    json_obj.push_str("}");
    Json(json_obj)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![version, get_avatar_by_id])
        .launch();

    // get_avatar_by_id(1)
}
