-- Your SQL goes here
CREATE TABLE feed_items (
    id                 SERIAL PRIMARY KEY,
    guid               VARCHAR UNIQUE NOT NULL,
    title              VARCHAR NOT NULL,
    link               VARCHAR NOT NULL,
    description        TEXT NOT NULL,
    published_at       TIMESTAMP NOT NULL,
    feed_channel_id    INTEGER REFERENCES feed_channels NOT NULL
)
