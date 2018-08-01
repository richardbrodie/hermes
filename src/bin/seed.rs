extern crate base64;
extern crate chrono;
extern crate diesel;
extern crate feeds_lib;
extern crate futures;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate sha2;
extern crate tokio;

use chrono::Utc;
use futures::prelude::*;

use feeds_lib::db::{self, insert_channel};
use feeds_lib::feed::fetch_feed;
use feeds_lib::models::FeedChannel;

fn main() {
  db::create_admin_user();

  let user = db::get_user("admin").unwrap();
  let work =
    add_feed("http://feeds.bbci.co.uk/news/rss.xml".to_owned(), user.id).and_then(move |_| {
      add_feed(
        "http://feeds.arstechnica.com/arstechnica/index".to_owned(),
        user.id,
      )
    });
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
      db::subscribe_channel(&uid, &ch_id);
      Ok(())
    })
}
