-- Your SQL goes here
CREATE VIEW subscribed_feeds_with_count_view AS
  SELECT f.*, s.user_id, CAST (unseen_count AS INTEGER)
  FROM feeds f
  INNER JOIN (
    SELECT i.feed_id, si.user_id, count(si.seen) AS unseen_count
    FROM items i
    INNER JOIN subscribed_items si
    ON i.id = si.item_id
    WHERE si.seen = 'f'
    GROUP BY i.feed_id, si.user_id
  ) s
  ON f.id = s.feed_id;
