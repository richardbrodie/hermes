use chrono::{DateTime, FixedOffset, Utc};
use futures::future::{self, IntoFuture};
use hyper::rt::{self, Future, Stream};
use hyper::Client;
use rss::{Channel, Item};
use std::io::BufReader;
use std::option::Option;
use std::str;
use std::time::{Duration, Instant};
use tokio::timer::Interval;

use db::{
  self, find_duplicates, get_channel_urls, insert_channel, insert_items, subscribe, update_item,
  NewItem,
};
use models::FeedChannel;

pub fn start_feed_loop() {
  let task = Interval::new(Instant::now(), Duration::from_secs(300))
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

pub fn subscribe_feed(url: String, uid: i32) {
  rt::spawn(future::lazy(move || {
    db::get_channel_id(&url)
      .into_future()
      .or_else(|_| add_feed(url))
      .and_then(move |c| Ok(subscribe(&uid, &c)))
  }));
}

pub fn add_feed(url: String) -> impl Future<Item = i32, Error = ()> {
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
      Ok((feed, channel.id))
    })
    .and_then(move |(feed, channel_id)| {
      let items: Vec<NewItem> = process_items(feed.items(), &channel_id);
      insert_items(&items);
      Ok(channel_id)
    })
}

pub fn update_feed(channel_id: i32, channel_url: String) {
  let work = fetch_feed(channel_url).and_then(move |feed| {
    let items: Vec<NewItem> = process_items(feed.items(), &channel_id);
    let new_items = process_duplicates(items);
    if new_items.len() > 0 {
      info!("{} new items", new_items.len());
      insert_items(&new_items);
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
            Ok(channel) => Ok(channel),
            Err(_e) => Err(()),
          },
        )
    })
}

fn process_items<'a>(feed_items: &'a [Item], channel_id: &'a i32) -> Vec<NewItem<'a>> {
  let items: Vec<NewItem> = feed_items
    .iter()
    .map(|item| NewItem {
      guid: item.guid().unwrap().value(),
      title: item.title().expect("no title!"),
      link: item.link().expect("no link!"),
      description: item.description().expect("no description!"),
      published_at: DateTime::<FixedOffset>::parse_from_rfc2822(item.pub_date().unwrap())
        .unwrap()
        .naive_local(),
      feed_channel_id: channel_id,
      content: item.content(),
    })
    .collect();
  items
}

fn process_duplicates(items: Vec<NewItem>) -> Vec<NewItem> {
  match find_duplicates(items.iter().map(|x| x.guid).collect()) {
    Some(dupes) => {
      let guids: Vec<&str> = dupes.iter().map(|x| x.1.as_str()).collect();
      let (new_items, mut duplicated_items): (Vec<NewItem>, Vec<NewItem>) =
        items.into_iter().partition(|x| !guids.contains(&x.guid));

      duplicated_items.into_iter().for_each(|d| {
        let idx = dupes.iter().find(|(_, y, _)| y == &d.guid).unwrap();
        if d.published_at != idx.2 {
          update_item(idx.0, &d)
        }
      });
      new_items
    }
    None => items,
  }
}
