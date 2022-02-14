use super::schema::avatars;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::get_data_uri_for_avatar;

#[derive(Queryable)]
pub struct Avatar {
    pub id: i32,
    pub mimetype: String,
    pub image: String,
    pub created: String,
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