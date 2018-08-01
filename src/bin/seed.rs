extern crate base64;
extern crate chrono;
extern crate diesel;
extern crate feeds_lib;
extern crate futures;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate sha2;
extern crate tokio;

use base64::encode;
use chrono::Utc;
use diesel::prelude::*;
use futures::prelude::*;
use sha2::{Digest, Sha256};
use std::str;

use feeds_lib::db::{self, establish_pool, insert_channel};
use feeds_lib::feed::fetch_feed;
use feeds_lib::models::{FeedChannel, User};
use feeds_lib::schema::feed_channels::dsl::*;
use feeds_lib::schema::feed_items::dsl::*;
use feeds_lib::schema::subscriptions::dsl::*;
use feeds_lib::schema::users::dsl::*;

fn main() {
  let userpass = "admin";

  let pool = establish_pool();
  let connection = pool.get().unwrap();

  diesel::delete(subscriptions).execute(&*connection).unwrap();
  diesel::delete(feed_items).execute(&*connection).unwrap();
  diesel::delete(feed_channels).execute(&*connection).unwrap();
  diesel::delete(users).execute(&*connection).unwrap();

  let pwh = hash_pw(&userpass);

  let res = diesel::insert_into(users)
    .values((username.eq(userpass), password_hash.eq(&pwh.as_bytes())))
    .load::<User>(&*connection)
    .expect("Error inserting to db");

  println!("user: {:?}", res[0].id);

  let work = add_feed("http://feeds.bbci.co.uk/news/rss.xml".to_owned(), res[0].id);
  tokio::run(work);
}

fn add_feed(url: String, uid: i32) -> impl Future<Item = (), Error = ()> {
  fetch_feed(url.to_string())
    .and_then(move |feed| {
      let mut channel = FeedChannel {
        id: 0,
        title: feed.title().to_string(),
        site_link: feed.link().to_string(),
        feed_link: url.to_string(),
        description: feed.description().to_string(),
        updated_at: Utc::now().naive_local(),
      };
      insert_channel(&mut channel);
      Ok(channel.id)
    })
    .and_then(move |ch_id| {
      db::subscribe(&uid, &ch_id);
      Ok(())
    })
}

fn hash_pw(s: &str) -> String {
  let mut hasher = Sha256::default();
  hasher.input(s.as_bytes());
  let output = hasher.result();
  let hash = &output[..];
  let e = encode(hash);
  e
}
