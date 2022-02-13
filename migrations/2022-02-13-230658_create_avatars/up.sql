CREATE TABLE avatars (
                       id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                       mimetype VARCHAR NOT NULL,
                       image TEXT NOT NULL,
                       created VARCHAR NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);