use base64::{decode, encode};
use chrono::NaiveDateTime;
use sha2::{Digest, Sha256};
use std::str;

use db::get_user;
use schema::*;

//////////
// Feed //
//////////

#[derive(Debug, Queryable, Associations, Identifiable, Serialize)]
pub struct Feed {
  pub id: i32,
  pub title: String,
  pub description: String,
  pub site_link: String,
  pub feed_link: String,
  pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "feeds"]
pub struct NewFeed {
  pub title: String,
  pub description: String,
  pub site_link: String,
  pub feed_link: String,
  pub updated_at: NaiveDateTime,
}

//////////
// Item //
//////////

#[derive(Debug, Queryable, Associations, Identifiable, Serialize)]
#[belongs_to(Feed)]
pub struct Item {
  pub id: i32,
  #[serde(skip_serializing)]
  pub guid: String,
  pub link: String,
  pub title: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub summary: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,
  pub published_at: NaiveDateTime,
  pub updated_at: Option<NaiveDateTime>,
  #[serde(skip_serializing)]
  pub feed_id: i32,
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "items"]
pub struct NewItem {
  pub guid: String,
  pub link: String,
  pub title: String,
  pub summary: Option<String>,
  pub content: Option<String>,
  pub published_at: NaiveDateTime,
  pub updated_at: Option<NaiveDateTime>,
  pub feed_id: i32,
}
// impl<'a> NewItem<'a> {
//   pub fn from_item(item: &Item) -> NewItem {
//     NewItem {
//       guid: item.guid,
//       title: item.title,
//       link: item.link,
//       summary: item.summary.and_then(|v| Some(v.as_str())),
//       content: item.content.and_then(|v| Some(v.as_str())),
//       published_at: item.published_at,
//       updated_at: item.updated_at.or(None),
//       feed_id: &item.feed_id,
//     }
//   }
// }

//////////////////
// Subscription //
//////////////////

#[derive(Debug, Queryable, Associations, Identifiable, Serialize, AsChangeset)]
#[belongs_to(Item)]
pub struct SubscribedItem {
  pub id: i32,
  pub item_id: i32,
  pub user_id: i32,
  pub seen: bool,
}

#[derive(Debug, Queryable, Associations, Identifiable, Serialize)]
#[belongs_to(Feed)]
pub struct SubscribedFeed {
  pub id: i32,
  pub user_id: i32,
  pub feed_id: i32,
}

///////////////
// Composite //
///////////////

#[derive(Debug, Queryable, Serialize)]
pub struct CompositeItem {
  pub item_id: i32,
  pub title: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub link: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub summary: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,
  pub published_at: NaiveDateTime,
  pub seen: bool,
}
impl CompositeItem {
  pub fn partial(item: (i32, String, Option<String>, NaiveDateTime, bool)) -> Self {
    CompositeItem {
      item_id: item.0,
      title: item.1.to_string(),
      link: None,
      summary: item.2,
      content: None,
      published_at: item.3,
      seen: item.4,
    }
  }
}

//////////
// User //
//////////

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

  pub fn hash_pw(s: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(s.as_bytes());
    let output = hasher.result();
    let hash = &output[..];
    let e = encode(hash);
    e
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

////////////
// Claims //
////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub name: String,
  pub id: i32,
}
