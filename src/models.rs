use argon2rs::verifier::Encoded;
use chrono::NaiveDateTime;

use db::get_user;
use schema::{feed_channels, feed_items, users};

#[derive(Debug, Queryable, Associations, Identifiable, Serialize)]
#[belongs_to(FeedChannel)]
pub struct FeedItem {
  pub id: i32,
  pub guid: String,
  pub title: String,
  pub link: String,
  pub description: String,
  pub published_at: NaiveDateTime,
  pub feed_channel_id: i32,
  pub content: Option<String>,
}

#[derive(Debug, Queryable, Associations, Identifiable, Serialize)]
pub struct FeedChannel {
  pub id: i32,
  pub title: String,
  pub site_link: String,
  pub feed_link: String,
  pub description: String,
  pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub password_hash: String,
}
impl User {
  pub fn check_user(username: &str, pass: &str) -> bool {
    match get_user(username) {
      Some(user) => user.verifies(pass),
      None => false,
    }
  }

  fn verifies(&self, pass: &str) -> bool {
    let enc0 = Encoded::from_u8(self.password_hash.as_bytes()).unwrap();
    enc0.verify(pass.as_bytes())
  }
}
