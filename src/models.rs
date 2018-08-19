use atom_syndication;
use base64::{decode, encode};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use rss;
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
  pub description: Option<String>,
  pub site_link: String,
  pub feed_link: String,
  pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "feeds"]
pub struct NewFeed {
  pub title: String,
  pub description: Option<String>,
  pub site_link: String,
  pub feed_link: String,
  pub updated_at: NaiveDateTime,
}
impl NewFeed {
  pub fn from_rss(feed: &rss::Channel, url: &str) -> NewFeed {
    NewFeed {
      title: feed.title().to_string(),
      site_link: feed.link().to_string(),
      feed_link: url.to_string(),
      description: Some(feed.description().to_string()),
      updated_at: Utc::now().naive_local(),
    }
  }

  pub fn from_atom(feed: &atom_syndication::Feed, url: &str) -> NewFeed {
    NewFeed {
      title: feed.title().to_string(),
      site_link: feed.links()[0].href().to_string(),
      feed_link: url.to_string(),
      description: feed.subtitle().and_then(|s| Some(s.to_owned())),
      updated_at: Utc::now().naive_local(),
    }
  }
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
  pub published_at: Option<NaiveDateTime>,
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
  pub published_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>,
  pub feed_id: i32,
}
impl NewItem {
  pub fn from_item(item: &rss::Item, feed_id: i32) -> NewItem {
    NewItem {
      guid: item.guid().unwrap().value().to_owned(),
      title: item.title().expect("no title!").to_owned(),
      link: item.link().expect("no link!").to_owned(),
      summary: item.description().and_then(|s| Some(s.to_owned())),
      content: item.content().and_then(|s| Some(s.to_owned())),
      published_at: parse_date(item.pub_date()),
      updated_at: parse_date(item.pub_date()),
      feed_id: feed_id,
    }
  }
  pub fn from_entry(item: &atom_syndication::Entry, feed_id: i32) -> NewItem {
    NewItem {
      guid: item.id().to_owned(),
      title: item.title().to_owned(),
      link: item.links()[0].href().to_owned(),
      summary: item.summary().and_then(|s| Some(s.to_owned())),
      content: item
        .content()
        .and_then(|o| o.value().and_then(|s| Some(s.to_owned()))),
      published_at: parse_date(item.published()),
      updated_at: parse_date_int(item.updated()),
      feed_id: feed_id,
    }
  }
}

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
  pub published_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>,
  pub seen: bool,
}
impl CompositeItem {
  pub fn partial(
    item: (
      i32,
      String,
      Option<String>,
      Option<NaiveDateTime>,
      Option<NaiveDateTime>,
      bool,
    ),
  ) -> Self {
    CompositeItem {
      item_id: item.0,
      title: item.1.to_string(),
      link: None,
      summary: item.2,
      content: None,
      published_at: item.3,
      updated_at: item.4,
      seen: item.5,
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

fn parse_date_int(date: &str) -> Option<NaiveDateTime> {
  match DateTime::parse_from_rfc2822(date) {
    Ok(d) => Some(d.naive_local()),
    Err(_) => match DateTime::parse_from_rfc3339(date) {
      Ok(d) => Some(d.naive_local()),
      Err(_) => None,
    },
  }
}
fn parse_date(date: Option<&str>) -> Option<NaiveDateTime> {
  match date {
    None => return None,
    Some(s) => parse_date_int(s),
  }
}
