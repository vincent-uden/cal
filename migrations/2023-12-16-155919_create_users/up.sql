-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    CONSTRAINT name_unique UNIQUE (username)
);
