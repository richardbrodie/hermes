use chrono::NaiveDateTime;

use schema::{feed_channels, feed_items};

#[derive(Debug, Queryable, Associations, Identifiable)]
#[belongs_to(FeedChannel)]
pub struct FeedItem {
  pub id: i32,
  pub guid: String,
  pub title: String,
  pub link: String,
  pub description: String,
  pub published_at: NaiveDateTime,
  pub feed_channel_id: i32,
}

#[derive(Debug, Queryable, Associations, Identifiable)]
pub struct FeedChannel {
  pub id: i32,
  pub title: String,
  pub link: String,
  pub description: String,
  pub updated_at: NaiveDateTime,
}
