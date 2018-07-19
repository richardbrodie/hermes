extern crate chrono;
extern crate diesel;
extern crate feeds_lib;
extern crate futures;
extern crate sodiumoxide;
extern crate tokio;

use chrono::Utc;
use diesel::prelude::*;
use futures::prelude::*;
use sodiumoxide::crypto::pwhash;

use feeds_lib::db::{self, establish_connection, insert_channel};
use feeds_lib::feed::fetch_feed;
use feeds_lib::models::{FeedChannel, User};
use feeds_lib::schema::feed_channels::dsl::*;
use feeds_lib::schema::subscriptions::dsl::*;
use feeds_lib::schema::users::dsl::*;

fn main() {
  let userpass = "admin";

  let connection = establish_connection();
  diesel::delete(subscriptions).execute(&connection).unwrap();
  diesel::delete(feed_channels).execute(&connection).unwrap();
  diesel::delete(users).execute(&connection).unwrap();

  let pwh = pwhash::pwhash(
    userpass.as_bytes(),
    pwhash::OPSLIMIT_INTERACTIVE,
    pwhash::MEMLIMIT_INTERACTIVE,
  ).unwrap();

  let res = diesel::insert_into(users)
    .values((username.eq(userpass), password_hash.eq(&pwh[..])))
    .load::<User>(&connection)
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
