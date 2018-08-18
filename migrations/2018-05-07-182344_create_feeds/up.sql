-- Your SQL goes here
CREATE TABLE feeds (
    id            SERIAL PRIMARY KEY,
    title         VARCHAR NOT NULL,
    description   TEXT NOT NULL,
    site_link     VARCHAR NOT NULL,
    feed_link     VARCHAR NOT NULL,
    updated_at    TIMESTAMP NOT NULL
)
