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
    let connection = establish_connection(None);
    let avatar_result = get_avatar_by_id_with_connection(&connection, query_id);
    let avatar_info = AvatarInfo::from(&avatar_result);
    let json_obj = serde_json::to_string(&avatar_info).unwrap();
    Json(json_obj)
}

fn get_avatar_by_id_with_connection(conn: &SqliteConnection, query_id: i32) -> Avatar {
    use crate::schema::avatars::dsl::*;

    let results = avatars
        .filter(id.eq(&query_id))
        .limit(1)
        .load::<Avatar>(conn)
        .expect("Error loading avatar");

    let avatar_result = results
        .into_iter()
        .next()
        .expect("Expected at least one item within the results");

    avatar_result
}

fn get_all_avatars_with_connection(conn: &SqliteConnection) -> Vec<Avatar> {
    use crate::schema::avatars::dsl::*;

    let results = avatars
        .load::<Avatar>(conn)
        .expect("Error listing all Avatars");

    results
}

fn main() {
    let connection = establish_connection(None);
    rocket::ignite()
        .mount("/", routes![version, get_avatar_by_id])
        .launch();
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
