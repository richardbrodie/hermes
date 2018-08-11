-- Your SQL goes here
CREATE TABLE subscribed_feed_items (
  id                 SERIAL PRIMARY KEY,
  user_id             INTEGER REFERENCES users NOT NULL,
  feed_item_id        INTEGER REFERENCES feed_items NOT NULL,
  seen                BOOLEAN NOT NULL DEFAULT false,
  UNIQUE(user_id, feed_item_id)
);

