-- Your SQL goes here
CREATE TABLE subscribed_feeds (
  id                 SERIAL PRIMARY KEY,
  user_id            INTEGER REFERENCES users NOT NULL,
  feed_id            INTEGER REFERENCES feeds NOT NULL,
  UNIQUE(user_id, feed_id)
);

