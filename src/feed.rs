use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use reqwest;
use rss::{Channel, Item};
use std::io::BufReader;

use db::{insert_channel, insert_items};
use models::{FeedChannel, FeedItem};

pub fn add_feed(url: &str) {
  let feed = fetch_feed(url).unwrap();
  let mut channel = FeedChannel {
    id: 0,
    title: feed.title().to_string(),
    link: feed.link().to_string(),
    description: feed.description().to_string(),
    updated_at: DateTime::<FixedOffset>::parse_from_rfc2822(feed.pub_date().unwrap())
      .unwrap()
      .naive_local(),
  };
  insert_channel(&mut channel);
  process_items(feed.items(), channel.id);
}

pub fn refresh_feed(channel: &FeedChannel) {
  let feed = fetch_feed(&channel.link).unwrap();
  process_items(feed.items(), channel.id);
}

pub fn fetch_feed(url: &str) -> Result<Channel, reqwest::Error> {
  let body = reqwest::get(url)?;
  let mut channel = Channel::read_from(BufReader::new(body)).unwrap();
  channel.set_items(vec![Item::default()]);
  Ok(channel)
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
