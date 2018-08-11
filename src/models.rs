use base64::{decode, encode};
use chrono::NaiveDateTime;
use sha2::{Digest, Sha256};
use std::str;

use db::get_user;
use schema::*;

#[derive(Debug, Queryable, Associations, Identifiable, Serialize)]
#[belongs_to(FeedChannel)]
pub struct FeedItem {
  pub id: i32,
  #[serde(skip_serializing)]
  pub guid: String,
  pub title: String,
  pub link: String,
  pub description: String,
  pub published_at: NaiveDateTime,
  #[serde(skip_serializing)]
  pub feed_channel_id: i32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,
}

#[derive(Debug, Queryable, Identifiable, Serialize)]
pub struct SubscribedFeedItem {
  pub id: i32,
  pub title: String,
  pub link: String,
  pub description: String,
  pub published_at: NaiveDateTime,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,
  pub seen: bool,
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

// #[derive(Debug, Queryable, Associations, Identifiable, Serialize)]
// pub struct SubscribedFeedChannel {
//   pub id: i32,
//   pub title: String,
//   pub site_link: String,
//   pub feed_link: String,
//   pub description: String,
//   pub updated_at: NaiveDateTime,
// }

#[derive(Debug, Queryable, Associations, Identifiable, Serialize)]
#[belongs_to(FeedChannel)]
pub struct Subscription {
  pub id: i32,
  pub user_id: i32,
  pub feed_channel_id: i32,
}

#[derive(Debug, Queryable, Associations, Identifiable, Serialize)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub password_hash: Vec<u8>,
}
impl User {
  pub fn check_user(username: &str, pass: &str) -> Option<User> {
    match get_user(username) {
      Some(user) => match user.verifies(pass) {
        true => Some(user),
        false => None,
      },
      None => None,
    }
  }

  fn verifies(&self, pass: &str) -> bool {
    let orig_hash = decode(&self.password_hash).unwrap();
    let mut hasher = Sha256::default();
    hasher.input(pass.as_bytes());
    let output = hasher.result();
    let hashed_pw = &output[..];
    orig_hash == hashed_pw
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub name: String,
  pub id: i32,
}
