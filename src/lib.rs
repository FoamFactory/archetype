#![feature(decl_macro)]
#![feature(in_band_lifetimes)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
extern crate dotenv;

use std::io::Write;
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use infer::Type;
use rocket::Data;
use rocket::data::ToByteUnit;
use crate::models::Avatar;
use crate::responders::RequestError;

pub mod util;
pub mod db;
pub mod schema;
pub mod models;
pub mod responders;
pub mod guards;

// Endpoint Utility Functions
pub async fn get_file_as_base64_encoded_string(file: Data<'_>) -> Result<(String, Type), RequestError> {
    let mut buffer: Vec<u8> = vec![];
    let written = file.open(512.kibibytes())
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

    let mut enc = base64::write::EncoderStringWriter::new(base64::STANDARD);
    let encoding_result = enc.write_all(buffer.as_slice());
    if encoding_result.is_err() {
        error_msg = Some("Unable to encode to base64 string");
    }

    if error_msg.is_some() {
        return Err(responders::RequestError::from((415, error_msg.unwrap())));
    }

    let b64_string = enc.into_inner();

    Ok((b64_string, kind))
}

pub fn delete_avatar_by_id_with_connection(conn: &MysqlConnection, query_id: i32) {
    use crate::schema::avatars::dsl::*;

    let _deleted_rows = diesel::delete(avatars.filter(id.eq(query_id))).execute(conn);
}

pub fn get_avatar_by_id_with_connection(conn: &MysqlConnection, query_id: i32) -> Result<Avatar, RequestError> {
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

pub fn get_all_avatars_with_connection(conn: &MysqlConnection) -> Vec<Avatar> {
    use crate::schema::avatars::dsl::*;

    let results = avatars
        .load::<Avatar>(conn)
        .expect("Error listing all Avatars");

    results
}

pub fn update_avatar_by_id_with_connection(conn: &MysqlConnection, query_id: i32, with_image: String, with_mime_type: String) -> Result<Avatar, RequestError> {
    use crate::schema::avatars::dsl::*;

    let update_result = diesel::update(avatars.filter(id.eq(query_id)))
        .set((image.eq(with_image), mimetype.eq(with_mime_type)))
        .execute(conn);

    if update_result.is_err() {
        return Err(RequestError::from((404, format!("Unable to find avatar with id {}", query_id).as_str())));
    }

    let result = avatars
        .filter(id.eq(query_id))
        .limit(1)
        .load::<Avatar>(conn);

    if result.is_err() {
        return Err(RequestError::from((500, format!("Database encountered an error while trying to update: {}", result.err().unwrap().to_string()).as_str())));
    }

    let avatar = result.unwrap()
        .into_iter()
        .next();

    if avatar.is_none() {
        return Err(RequestError::from((404, format!("Unable to find avatar with id {}", query_id).as_str())));
    }

    return Ok(avatar.unwrap())
}