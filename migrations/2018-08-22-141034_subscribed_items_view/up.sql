-- Your SQL goes here
CREATE VIEW subscribed_items_view AS
  SELECT i.*, s.id as subscribed_item_id, s.user_id, s.seen
  FROM items i
  INNER JOIN subscribed_items s
  ON i.id = s.item_id;
