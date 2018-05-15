use chrono::{DateTime, FixedOffset, Utc};
use futures::{Future, Stream};
use hyper::{Client, Error};
use rss::{Channel, Item};
use std::io::BufReader;
use std::str;
use tokio_core::reactor::Core;

use db::{insert_channel, insert_items};
use models::{FeedChannel, FeedItem};

pub fn add_feed(url: &str) {
  let feed = fetch_feed(url).unwrap();
  let mut channel = FeedChannel {
    id: 0,
    title: feed.title().to_string(),
    link: feed.link().to_string(),
    description: feed.description().to_string(),
    updated_at: Utc::now().naive_local(),
  };
  insert_channel(&mut channel);
  process_items(feed.items(), channel.id);
}

pub fn refresh_feed(channel: &FeedChannel) {
  let feed = fetch_feed(&channel.link).unwrap();
  process_items(feed.items(), channel.id);
}

pub fn fetch_feed(uri: &str) -> Result<Channel, Error> {
  let mut core = Core::new()?;
  let client = Client::new(&core.handle());

  let work = client.get(uri.parse()?).and_then(|res| {
    res.body().concat2().and_then(move |body| {
      let s = Channel::read_from(BufReader::new(&body as &[u8])).unwrap();
      Ok(s)
    })
  });
  let res = core.run(work)?;
  Ok(res)
}

fn process_items(feed_items: &[Item], channel_id: i32) {
  let items: Vec<FeedItem> = feed_items
    .iter()
    .map(|item| FeedItem {
      id: 0,
      guid: item.guid().unwrap().value().to_string(),
      title: item.title().expect("no title!").to_string(),
      link: item.link().expect("no link!").to_string(),
      description: item.description().expect("no description!").to_string(),
      published_at: DateTime::<FixedOffset>::parse_from_rfc2822(item.pub_date().unwrap())
        .unwrap()
        .naive_local(),
      feed_channel_id: channel_id,
    })
    .collect();
  insert_items(&items);
}
