#![feature(decl_macro)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
extern crate dotenv;

use std::io::Write;
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use infer::Type;
use rocket::Data;
use rocket::data::{ByteUnit};
use crate::models::Avatar;
use crate::responders::RequestError;

pub mod util;
pub mod db;
pub mod schema;
pub mod models;
pub mod responders;
pub mod guards;

// Endpoint Utility Functions
pub async fn get_file_as_base64_encoded_string(file: Data<'_>, limit: ByteUnit) -> Result<(String, Type), RequestError> {
    let mut buffer: Vec<u8> = vec![];
    let written = file.open(limit)
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
        .load::<Avatar>(conn);

    if results.is_err() {
        return Err(RequestError::from((404, format!("Unable to find avatar with id {}", query_id).as_str())));
    }

    let avatar_result = results
        .into_iter()
        .next();

    if avatar_result.is_none() {
        return Err(RequestError::from((404, format!("No avatar found with id: {}", query_id).as_str())));
    }

    let result_unwrapped = avatar_result.unwrap();
    if result_unwrapped.len() == 0 {
        return Err(RequestError::from((404, format!("No avatar found with id: {}", query_id).as_str())));
    }

    let avatar = result_unwrapped.into_iter().next();
    Ok(avatar.unwrap())
}

pub fn get_all_avatars_with_connection(conn: &MysqlConnection) -> Result<Vec<Avatar>, RequestError> {
    use crate::schema::avatars::dsl::*;

    let results: Result<Vec<Avatar>, _> = avatars
        .load::<Avatar>(conn);

    if results.is_err() {
        return Err(RequestError::from((400, "Unable to retrieve all avatars")));
    }

    Ok(results.unwrap())
}

pub fn update_avatar_by_id_with_connection(conn: &MysqlConnection, query_id: i32, with_image: String, with_mime_type: String) -> Result<Avatar, RequestError> {
    use crate::schema::avatars::dsl::*;

    let binary_image: Vec<u8> = base64::decode(with_image).unwrap();
    let update_result = diesel::update(avatars.filter(id.eq(query_id)))
        .set((image.eq(binary_image), mimetype.eq(with_mime_type)))
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