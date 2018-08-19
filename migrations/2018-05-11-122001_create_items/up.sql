-- Your SQL goes here
CREATE TABLE items (
    id                 SERIAL PRIMARY KEY,
    guid               VARCHAR UNIQUE NOT NULL,
    link               VARCHAR NOT NULL,
    title              VARCHAR NOT NULL,
    summary            TEXT,
    content            TEXT,
    published_at       TIMESTAMPTZ,
    updated_at         TIMESTAMPTZ,
    feed_id            INTEGER REFERENCES feeds NOT NULL
)
