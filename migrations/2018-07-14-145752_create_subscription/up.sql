-- Your SQL goes here
CREATE TABLE subscriptions (
    id                 SERIAL PRIMARY KEY,
    user_id            INTEGER REFERENCES users NOT NULL,
    feed_channel_id    INTEGER REFERENCES feed_channels NOT NULL
)
