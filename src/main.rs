#[macro_use] extern crate rocket;

use diesel::MysqlConnection;
use rocket::data::Data;
// use rocket::serde::json::Json;
use rocket::response::content::Json;
use rocket::{Build, catchers, Request, Rocket, routes};
use archetype_lib::db::establish_connection;
use archetype_lib::{delete_avatar_by_id_with_connection, get_all_avatars_with_connection, get_avatar_by_id_with_connection, get_file_as_base64_encoded_string, responders, update_avatar_by_id_with_connection};
use archetype_lib::models::{Avatar, AvatarInfo, AvatarUri, DehydratedAvatarInfo, ResponseMessage, VersionInfo};
use archetype_lib::util::{extract_data_from_uri, get_version_code_from_string};

use archetype_lib::guards::AllowedHosts;
use archetype_lib::responders::JsonRetriever;

// Endpoint Definitions
#[get("/version")]
fn version(_allowed_hosts: AllowedHosts) -> Json<String> {
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
fn delete_avatar_by_id(query_id: i32, _allowed_hosts: AllowedHosts) -> Json<String> {
    let connection = establish_connection(None);
    delete_avatar_by_id_with_connection(&connection, query_id);
    let response_message = ResponseMessage {
        message: format!("Avatar with id {} deleted", query_id)
    };
    let json_obj = serde_json::to_string(&response_message).unwrap();
    Json(json_obj)
}

#[get("/avatar/<query_id>")]
fn get_avatar_by_id(query_id: i32, _allowed_hosts: AllowedHosts) -> Result<Json<String>, responders::RequestError> {
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

#[put("/avatar/<query_id>", data = "<upload_file>")]
async fn put_avatar(query_id: i32, upload_file: Data<'_>, _allowed_hosts: AllowedHosts) -> Result<Json<String>, responders::RequestError> {
    let (b64_string, kind) = get_file_as_base64_encoded_string(upload_file).await?;

    // Connect to the database
    let conn: MysqlConnection = establish_connection(None);

    let avt: Avatar = update_avatar_by_id_with_connection(&conn,
                                                          query_id,
                                                          b64_string,
                                                          kind.mime_type().to_string())?;
    let info = AvatarInfo::from(&avt);
    let json_obj = serde_json::to_string(&info).unwrap();
    Ok(Json(json_obj))
}

#[get("/avatars")]
fn get_all_avatars(_allowed_hosts: AllowedHosts) -> Result<Json<String>, responders::RequestError> {
    // Connect to the database
    let conn: MysqlConnection = establish_connection(None);

    let avts: Vec<Avatar> = get_all_avatars_with_connection(&conn)?;

    let av_infos: Vec<DehydratedAvatarInfo> = avts.into_iter()
        .map(|av| DehydratedAvatarInfo::from(&av))
        .collect();
    let json_obj = serde_json::to_string(&av_infos).unwrap();
    Ok(Json(json_obj))
}

#[post("/avatar/uri", format = "json", data = "<avatar_uri>")]
async fn upload_new_avatar_from_uri(avatar_uri: rocket::serde::json::Json<AvatarUri>, _allowed_hosts: AllowedHosts) -> Result<Json<String>, responders::RequestError>  {
    let (mime_type, data) = extract_data_from_uri(&avatar_uri.data_uri);

    // Connect to the database
    let conn: MysqlConnection = establish_connection(None);

    // Create a new Avatar object and put it in the database
    let avt: Avatar = Avatar::create(&mime_type, &data, &conn);

    let info = AvatarInfo::from(&avt);
    let json_obj = serde_json::to_string(&info).unwrap();
    Ok(Json(json_obj))
}

#[post("/avatar/file", data = "<upload_file>")]
async fn upload_new_avatar_from_file(upload_file: Data<'_>, _allowed_hosts: AllowedHosts) -> Result<Json<String>, responders::RequestError>  {
    let (b64_string, kind) = get_file_as_base64_encoded_string(upload_file).await?;

    // Connect to the database
    let conn: MysqlConnection = establish_connection(None);

    // Create a new Avatar object and put it in the database
    let avt: Avatar = Avatar::create(kind.mime_type(), &b64_string, &conn);

    let info = AvatarInfo::from(&avt);
    let json_obj = serde_json::to_string(&info).unwrap();
    Ok(Json(json_obj))
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

#[catch(404)]
async fn not_found(req: &Request<'_>) -> Json<String> {
    let def_req_error = r#"
    {
        "message": "Path not found"
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
    rocket::build()
        .mount("/", routes![
            version,
            delete_avatar_by_id,
            get_avatar_by_id,
            upload_new_avatar_from_file,
            upload_new_avatar_from_uri,
            put_avatar,
            get_all_avatars,
        ])
        .register("/", catchers![forbidden, not_found])
}

// TODO_jwir3: Tests need to be rewritten to use MySQL since we can't use BOTH MySQL and Sqlite
//             connections and change only at runtime.

// #[cfg(test)]
// mod tests {
//     use diesel::{delete, RunQueryDsl};
//     use archetype_lib::get_all_avatars_with_connection;
//     use archetype_lib::schema::avatars::dsl::*;
//     // use crate::schema::avatars::dsl::*;
//     use crate::{Avatar, establish_connection, get_avatar_by_id, get_avatar_by_id_with_connection};
//
//     struct TestContext {
//         avatar_ids_created: Vec<i32>
//     }
//
//     impl TestContext {
//         fn new() -> Self {
//             let connection = establish_connection(Some(String::from("test.db")));
//             let inserted_avs = Avatar::create("image/png", "iVBORw0KGgoAAAANSUhEUgAAAAoAAAAKCAIAAAACUFjqAAABj2lDQ1BJQ0MgcHJvZmlsZQAAKJF9kb1Lw1AUxU9TxSotIjqIdMhQHcSCKIiDi1UsQoVSK1h1MHnph5CkIUlxcRRcCw6ii1+D/gGiq4OrIAiKIOLkH+DXIiXelwRapPXB4/5ycu/hvfMA4VxlmtU2Cmi6bWaSCXE5tyJ2vEFABD0Io1NiljGdTqfQcn0/IMDrfZx7te5ruiJK3mJAQCSeY4ZpE5eIJzZtg/MRcR8rSQrxBfGISQckfuW67PEn56LLQoizmc3MEEeJxWIDyw3MSqZGPEUcUzSd/IV1jxXO25w1tcL8c/IbhvP60iLVYdpRJKFiAxoMWMhDhIwKfauwEaeqk2IhQ10Jyra5z4Drk6Y52fViNDOLMnlKrgP4W/zN2CqMj3lOYXJuf3Gcj0GgYxeoVR3n59hxaidA8Bm41uvzZcpx8ov0al2LHQLddM/Lm7om7wFXO0D/kyGZkisFaQuFAvB+Rs+VA3rvgK5VLz//P04fgewWkLoF9g+AoSJ5r7W4d8jPbx4LSP/b4yf4C4vSdIzDS13OAAAACXBIWXMAAC4jAAAuIwF4pT92AAAAB3RJTUUH5gINEQwu6eRSBgAAABx0RVh0Q29tbWVudABDcmVhdGVkIHdpdGggR2xpbXBzZe5OGAcAAAAUSURBVBjTY/x/jgEPYGJgGJXGBAA5EwHhXz/1YwAAAABJRU5ErkJggg==", &connection);
//             let mut av_ids = vec![inserted_avs.id];
//             Self {
//                 avatar_ids_created: av_ids
//             }
//         }
//
//         fn get_last_created_avatar_id(&self) -> i32 {
//             let last_av_id = self.avatar_ids_created
//                 .iter()
//                 .rev()
//                 .next()
//                 .unwrap();
//             *last_av_id
//         }
//     }
//
//     impl Drop for TestContext {
//         fn drop(&mut self) {
//             let connection = establish_connection(Some(String::from("test.db")));
//             delete(avatars).execute(&connection);
//         }
//     }
//
//     #[test]
//     fn it_should_populate_the_avatars_table_with_one_entry() {
//         let context = TestContext::new();
//
//         let connection = establish_connection(Some(String::from("test.db")));
//         let avatars_vec = get_all_avatars_with_connection(&connection);
//
//         assert_eq!(1, avatars_vec.len());
//     }
//
//     #[test]
//     fn it_should_be_able_to_retrieve_an_avatar_from_the_database() {
//         let context = TestContext::new();
//
//         let connection = establish_connection(Some(String::from("test.db")));
//         // let avatars_vec = get_all_avatars_with_connection(connection);
//         let avatar_id = 1;
//
//         let all_avatars = get_all_avatars_with_connection(&connection);
//         println!("{:?}", all_avatars);
//
//         let avatar = get_avatar_by_id_with_connection(&connection, context.get_last_created_avatar_id());
//
//         assert_eq!(context.get_last_created_avatar_id(), avatar.id);
//     }
//
//     #[test]
//     fn it_should_respond_with_404_if_attempting_to_get_an_unknown_id() {
//         let context = TestContext::new();
//
//         let connection = establish_connection(Some(String::from("test.db")));
//         // let avatars_vec = get_all_avatars_with_connection(connection);
//         let avatar_id = 1;
//
//         let all_avatars = get_all_avatars_with_connection(&connection);
//         println!("{:?}", all_avatars);
//
//         let avatar = get_avatar_by_id_with_connection(&connection, context.get_last_created_avatar_id());
//
//         assert_eq!(context.get_last_created_avatar_id(), avatar.id);
//     }
// }
