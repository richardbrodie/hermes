-- Your SQL goes here
CREATE TABLE subscribed_items (
  id                  SERIAL PRIMARY KEY,
  user_id             INTEGER REFERENCES users NOT NULL,
  item_id             INTEGER REFERENCES items NOT NULL,
  seen                BOOLEAN NOT NULL DEFAULT false,
  UNIQUE(user_id, item_id)
);

