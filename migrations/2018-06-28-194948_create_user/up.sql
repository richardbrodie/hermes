-- Your SQL goes here
CREATE TABLE users (
    id                 SERIAL PRIMARY KEY,
    username           VARCHAR UNIQUE NOT NULL,
    password_hash      VARCHAR NOT NULL
)
