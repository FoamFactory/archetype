CREATE TABLE avatars (
                       id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
                       mimetype TEXT NOT NULL,
                       image LONGTEXT NOT NULL,
                       created DATETIME NOT NULL DEFAULT now()
);