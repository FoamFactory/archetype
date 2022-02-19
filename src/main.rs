#![feature(decl_macro)]
#![feature(in_band_lifetimes)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;

#[macro_use]
extern crate diesel;
use self::diesel::prelude::*;

extern crate dotenv;

use std::process::id;
use rocket::response::Debug;
use rocket::data::{Data, ToByteUnit};
use rocket::response::content::Json;
use std::io::{Error, Write};
use std::pin::Pin;
use std::task::{Context, Poll};
use base64::write::EncoderStringWriter;
use rocket::{Build, Request, Rocket};
use rocket::response::status;
use rocket::response::status::NotFound;

pub mod util;
pub mod db;
pub mod schema;
pub mod models;
pub mod responders;
pub mod guards;

use crate::guards::AllowedHosts;

use crate::util::{get_data_uri_for_avatar, get_version_code_from_string};

use db::*;
use crate::models::{Avatar, AvatarInfo, ResponseMessage, VersionInfo};
use crate::responders::{JsonRetriever, RequestError};

// Endpoint Definitions
#[get("/version")]
fn version(allowed_hosts: AllowedHosts) -> Json<String> {
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

#[delete("/avatar/<query_id>")]
fn delete_avatar_by_id(query_id: i32, allowed_hosts: AllowedHosts) -> Json<String> {
    let connection = establish_connection(None);
    delete_avatar_by_id_with_connection(&connection, query_id);
    let response_message = ResponseMessage {
        message: format!("Avatar with id {} deleted", query_id)
    };
    let json_obj = serde_json::to_string(&response_message).unwrap();
    Json(json_obj)
}

#[get("/avatar/<query_id>")]
fn get_avatar_by_id(query_id: i32, allowed_hosts: AllowedHosts) -> Result<Json<String>, responders::RequestError> {
    let connection = establish_connection(None);
    let avatar_result = get_avatar_by_id_with_connection(&connection, query_id);
    if avatar_result.is_err() {
        return Err(avatar_result.err().unwrap());
    }

    let avatar = avatar_result.unwrap();
    let avatar_info = AvatarInfo::from(&avatar);
    let json_obj = serde_json::to_string(&avatar_info).unwrap();
    Ok(Json(json_obj))
}

#[post("/avatar", data = "<upload_file>")]
async fn upload_new_avatar(upload_file: Data<'_>, allowed_hosts: AllowedHosts) -> Result<Json<String>, responders::RequestError>  {
    use responders::RequestErrorMessage;

    let mut buffer: Vec<u8> = vec![];
    let written = upload_file.open(512.kibibytes())
        .stream_to(&mut buffer).await;
    if written.is_err() {
        return Err(responders::RequestError::from((400, "File Corrupted")));
    }

    // Infer mime type
    let mut error_msg: Option<&'static str> = None;
    let kind_opt = infer::get(&buffer);
    if kind_opt.is_none() {
        error_msg = Some("Unable to infer mime type from given data");
    }

    let kind = kind_opt.unwrap();
    if kind.mime_type() != "image/jpeg" && kind.mime_type() != "image/png" {
        error_msg = Some("Image data sent is not in PNG or JPG format");
    }

    if error_msg.is_some() {
        return Err(responders::RequestError::from((415, error_msg.unwrap())));
    }

    let mut enc = base64::write::EncoderStringWriter::new(base64::STANDARD);
    enc.write_all(buffer.as_slice());
    let b64_string = enc.into_inner();

    // Connect to the database
    let conn: SqliteConnection = establish_connection(None);

    // Create a new Avatar object and put it in the database
    let avt: Avatar = Avatar::create(kind.mime_type(), &b64_string, &conn);

    let info = AvatarInfo::from(&avt);
    let json_obj = serde_json::to_string(&info).unwrap();
    Ok(Json(json_obj))
}

// Endpoint Utility Functions
fn delete_avatar_by_id_with_connection(conn: &SqliteConnection, query_id: i32) {
    use crate::schema::avatars::dsl::*;
    let deleted_rows = diesel::delete(avatars.filter(id.eq(query_id))).execute(conn);
}

fn get_avatar_by_id_with_connection(conn: &SqliteConnection, query_id: i32) -> Result<Avatar, RequestError> {
    use crate::schema::avatars::dsl::*;

    let results = avatars
        .filter(id.eq(&query_id))
        .limit(1)
        .load::<Avatar>(conn)
        .expect("Error loading avatar");

    if results.is_empty() {
        return Err(RequestError::from((404, format!("Unable to find avatar with id {}", query_id).as_str())));
    }

    let avatar_result = results
        .into_iter()
        .next()
        .expect("Expected at least one item within the results");

    Ok(avatar_result)
}

fn get_all_avatars_with_connection(conn: &SqliteConnection) -> Vec<Avatar> {
    use crate::schema::avatars::dsl::*;

    let results = avatars
        .load::<Avatar>(conn)
        .expect("Error listing all Avatars");

    results
}

// Error Catchers
#[catch(403)]
async fn forbidden(req: &Request<'_>) -> Json<String> {
    let def_req_error = r#"
    {
        message: "Access Denied"
    }
    "#;
    let mut json_string = String::from(def_req_error);
    let guard_result = req.guard::<AllowedHosts>().await;
    if guard_result.is_failure() {
        let status_gd_result= guard_result.failed().unwrap();
        let req_json = status_gd_result.1.get_json();
        json_string = String::from(&req_json.0);
    }
    Json(json_string)
}

// Main Function Replacement
#[launch]
fn rocket() -> Rocket<Build> {
    let connection = establish_connection(None);
    rocket::build()
        .mount("/", routes![
            version,
            delete_avatar_by_id,
            get_avatar_by_id,
            upload_new_avatar,
        ])
        .register("/", catchers![forbidden])
}

#[cfg(test)]
mod tests {
    use diesel::{delete, RunQueryDsl};
    use crate::schema::avatars::dsl::*;
    use crate::{Avatar, establish_connection, get_all_avatars_with_connection, get_avatar_by_id, get_avatar_by_id_with_connection};

    struct TestContext {
        avatar_ids_created: Vec<i32>
    }

    impl TestContext {
        fn new() -> Self {
            let connection = establish_connection(Some(String::from("test.db")));
            let inserted_avs = Avatar::create("image/png", "iVBORw0KGgoAAAANSUhEUgAAAAoAAAAKCAIAAAACUFjqAAABj2lDQ1BJQ0MgcHJvZmlsZQAAKJF9kb1Lw1AUxU9TxSotIjqIdMhQHcSCKIiDi1UsQoVSK1h1MHnph5CkIUlxcRRcCw6ii1+D/gGiq4OrIAiKIOLkH+DXIiXelwRapPXB4/5ycu/hvfMA4VxlmtU2Cmi6bWaSCXE5tyJ2vEFABD0Io1NiljGdTqfQcn0/IMDrfZx7te5ruiJK3mJAQCSeY4ZpE5eIJzZtg/MRcR8rSQrxBfGISQckfuW67PEn56LLQoizmc3MEEeJxWIDyw3MSqZGPEUcUzSd/IV1jxXO25w1tcL8c/IbhvP60iLVYdpRJKFiAxoMWMhDhIwKfauwEaeqk2IhQ10Jyra5z4Drk6Y52fViNDOLMnlKrgP4W/zN2CqMj3lOYXJuf3Gcj0GgYxeoVR3n59hxaidA8Bm41uvzZcpx8ov0al2LHQLddM/Lm7om7wFXO0D/kyGZkisFaQuFAvB+Rs+VA3rvgK5VLz//P04fgewWkLoF9g+AoSJ5r7W4d8jPbx4LSP/b4yf4C4vSdIzDS13OAAAACXBIWXMAAC4jAAAuIwF4pT92AAAAB3RJTUUH5gINEQwu6eRSBgAAABx0RVh0Q29tbWVudABDcmVhdGVkIHdpdGggR2xpbXBzZe5OGAcAAAAUSURBVBjTY/x/jgEPYGJgGJXGBAA5EwHhXz/1YwAAAABJRU5ErkJggg==", &connection);
            let mut av_ids = vec![inserted_avs.id];
            Self {
                avatar_ids_created: av_ids
            }
        }

        fn get_last_created_avatar_id(&self) -> i32 {
            let last_av_id = self.avatar_ids_created
                .iter()
                .rev()
                .next()
                .unwrap();
            *last_av_id
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            let connection = establish_connection(Some(String::from("test.db")));
            delete(avatars).execute(&connection);
        }
    }

    #[test]
    fn it_should_populate_the_avatars_table_with_one_entry() {
        let context = TestContext::new();

        let connection = establish_connection(Some(String::from("test.db")));
        let avatars_vec = get_all_avatars_with_connection(&connection);

        assert_eq!(1, avatars_vec.len());
    }

    #[test]
    fn it_should_be_able_to_retrieve_an_avatar_from_the_database() {
        let context = TestContext::new();

        let connection = establish_connection(Some(String::from("test.db")));
        // let avatars_vec = get_all_avatars_with_connection(connection);
        let avatar_id = 1;

        let all_avatars = get_all_avatars_with_connection(&connection);
        println!("{:?}", all_avatars);

        let avatar = get_avatar_by_id_with_connection(&connection, context.get_last_created_avatar_id());

        assert_eq!(context.get_last_created_avatar_id(), avatar.id);
    }
}
