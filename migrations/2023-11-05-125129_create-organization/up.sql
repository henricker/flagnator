-- Your SQL goes here
CREATE TABLE organizations (
    id TEXT PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL
);