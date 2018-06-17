use chrono::{DateTime, FixedOffset, Utc};
use hyper::rt::{self, Future, Stream};
use hyper::Client;
use rss::{Channel, Item};
use std::io::BufReader;
use std::str;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Interval;

use db::{
  channel_exists, get_channel, get_channel_urls, get_channels, get_latest_item_date,
  insert_channel, insert_items,
};
use models::{FeedChannel, FeedItem};

pub fn start_feed_loop() {
  let task = Interval::new(Instant::now(), Duration::from_secs(60))
    .for_each(|_| {
      get_channel_urls().into_iter().for_each(|c| {
        update_feed(c.0, c.1);
      });
      Ok(())
    })
    .map_err(|e| panic!("delay errored; err={:?}", e));

  info!("starting feed loop");
  rt::spawn(task);
}

pub fn add_feed(url: String) {
  info!("adding feed: {}", url);
  if channel_exists(&url) {
    info!("feed exists");
    return ();
  }

  let work = fetch_feed(url.to_string())
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
      info!("created new feed: {}", channel.title);
      Ok((feed, channel.id))
    })
    .and_then(|(feed, channel_id)| {
      let items: Vec<FeedItem> = process_items(feed.items(), channel_id);
      info!("inserting {} items", items.len());
      insert_items(&items);
      Ok(())
    });

  rt::spawn(work);
}

pub fn update_feed(channel_id: i32, channel_url: String) {
  let work = fetch_feed(channel_url)
    .and_then(move |feed| {
      let items: Vec<FeedItem> = process_items(feed.items(), channel_id);
      Ok(items)
    })
    .and_then(move |mut items| {
      match get_latest_item_date(channel_id) {
        Some(date) => items.retain(|ref x| x.published_at > date),
        None => (),
      };
      info!("found {} new items", items.len());
      if items.len() > 0 {
        insert_items(&items);
      }
      Ok(())
    });

  rt::spawn(work);
}

// internal

pub fn fetch_feed(url: String) -> impl Future<Item = Channel, Error = ()> {
  let client = Client::new();
  client
    .get(url.parse().unwrap())
    .map_err(|_err| ())
    .and_then(|res| {
      res
        .into_body()
        .concat2()
        .map_err(|_err| ())
        .and_then(
          |body| match Channel::read_from(BufReader::new(&body as &[u8])) {
            Ok(channel) => {
              info!("fetched feed: {:?}", channel.title());
              Ok(channel)
            }
            Err(_e) => Err(()),
          },
        )
    })
}

fn process_items(feed_items: &[Item], channel_id: i32) -> Vec<FeedItem> {
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
  items
}
