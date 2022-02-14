#![feature(decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;

#[macro_use]
extern crate diesel;
use self::diesel::prelude::*;

extern crate dotenv;

use std::process::id;
use rocket::response::content::Json;

pub mod util;
pub mod db;
pub mod schema;
pub mod models;

use crate::util::{get_data_uri_for_avatar, get_version_code_from_string};

use db::*;
use crate::models::{Avatar, AvatarInfo, VersionInfo};

// Endpoint Definitions
#[get("/version")]
fn version() -> Json<String> {
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");
    let version_info = VersionInfo {
        name: String::from(pkg_name),
        version: String::from(pkg_version),
        version_code: get_version_code_from_string(pkg_version)
    };

    let json_obj = serde_json::to_string(&version_info).unwrap();
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

    let avatar_info = AvatarInfo::from(&results[0]);
    let json_obj = serde_json::to_string(&avatar_info).unwrap();
    Json(json_obj)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![version, get_avatar_by_id])
        .launch();
}
