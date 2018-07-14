use chrono::NaiveDateTime;
use sodiumoxide::crypto::pwhash;

use db::get_user;
use schema::*;

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
    let stored_hash = pwhash::HashedPassword::from_slice(&self.password_hash).unwrap();
    pwhash::pwhash_verify(&stored_hash, pass.as_bytes())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub name: String,
  pub id: i32,
}
