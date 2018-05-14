-- Your SQL goes here
CREATE TABLE feed_channels (
    id            SERIAL PRIMARY KEY,
    title         VARCHAR NOT NULL,
    link          VARCHAR NOT NULL,
    description   TEXT NOT NULL,
    updated_at    TIMESTAMP NOT NULL
)
