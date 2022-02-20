use diesel::table;

table! {
    avatars (id) {
        id -> Integer,
        mimetype -> Text,
        image -> Text,
        created -> Text,
    }
}
