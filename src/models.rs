use super::schema::avatars;

#[derive(Queryable)]
pub struct Avatar {
    pub id: i32,
    pub mimetype: String,
    pub image: String,
    pub created: String,
}