#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content::Json;

pub mod util;
use crate::util::get_version_code_from_string;

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

fn main() {
    rocket::ignite()
        .mount("/", routes![version])
        .launch();
}
