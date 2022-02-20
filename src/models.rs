use diesel::{ExpressionMethods, insert_into, QueryDsl, RunQueryDsl, SqliteConnection};
use crate::schema::avatars;

use serde::{Deserialize, Serialize};
use crate::models::avatars::dsl::avatars as avatars_dsl;
use crate::util::get_data_uri_for_avatar;

#[derive(Debug, Queryable)]
pub struct Avatar {
    pub id: i32,
    pub mimetype: String,
    pub image: String,
    pub created: String,
}

impl Avatar {
    pub fn create(with_mimetype: &str, with_image: &str, conn: &SqliteConnection) -> Avatar {
        use crate::schema::avatars::dsl::*;

        let _inserted_count = insert_into(avatars_dsl)
            .values((mimetype.eq(String::from(with_mimetype)), image.eq(String::from(with_image))))
            .execute(conn)
            .expect("Error saving new Avatar record");

        let result = avatars_dsl
            .order(id.desc())
            .limit(1)
            .load::<Avatar>(conn)
            .unwrap()
            .into_iter()
            .next()
            .expect("Expected a single Avatar to be created");

        result
    }
}

#[derive(Serialize, Deserialize)]
pub struct AvatarInfo {
    pub id: i32,
    pub mimetype: String,
    pub image: String,
    pub created: String,
    pub data_uri: String,
}

impl From<&Avatar> for AvatarInfo {
    fn from(avatar: &Avatar) -> Self {
        AvatarInfo {
            id: avatar.id,
            mimetype: String::from(&avatar.mimetype),
            image: String::from(&avatar.image),
            created: String::from(&avatar.created),
            data_uri: get_data_uri_for_avatar(&avatar)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct VersionInfo {
    pub name: String,
    pub version_code: u32,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMessage {
    pub message: String
}