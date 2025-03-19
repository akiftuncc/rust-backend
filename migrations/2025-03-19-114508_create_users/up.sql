-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username varchar(64) NOT NULL UNIQUE,
    password varchar(128) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW() NOT NULL
)