-- Your SQL goes here
CREATE TABLE subscriptions (
    user_id            INTEGER REFERENCES users NOT NULL,
    feed_channel_id    INTEGER REFERENCES feed_channels NOT NULL,
    PRIMARY KEY(user_id, feed_channel_id)
);

