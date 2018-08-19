-- Your SQL goes here
CREATE TABLE feeds (
    id            SERIAL PRIMARY KEY,
    title         VARCHAR NOT NULL,
    description   TEXT,
    site_link     VARCHAR NOT NULL,
    feed_link     VARCHAR NOT NULL,
    updated_at    TIMESTAMPTZ NOT NULL
)
