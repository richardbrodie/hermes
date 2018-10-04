use atom_syndication;
use futures::future::IntoFuture;
use hyper::rt::{self, Future, Stream};
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use quick_xml::events::Event;
use quick_xml::Reader;
use rss;
use std::io::BufReader;
use std::option::Option;
use std::str;
use std::time::{Duration, Instant};
use tokio::timer::Interval;

use db::{
  self, find_duplicates, get_channel_urls_and_subscribers, insert_channel, insert_items,
  insert_subscribed_items, update_item,
};
use models::{CompositeItem, Item, NewFeed, NewItem, OutgoingWebsocketMessage};
use web::{types::SubscribeParams, types::UserWebsocketState, ws::ws_send_message};

enum FeedType {
  RSS(rss::Channel),
  Atom(atom_syndication::Feed),
}
enum ItemType {
  Item(Vec<rss::Item>),
  Entry(Vec<atom_syndication::Entry>),
}

////////////////////////
/// Future sequences ///
////////////////////////

pub fn start_interval_loops(global_user_state: UserWebsocketState) {
  let update_subscriptions = Interval::new(Instant::now(), Duration::from_secs(300))
    .for_each(move |_| {
      get_channel_urls_and_subscribers().into_iter().for_each(
        |(feed_id, feed_url, subscriber_ids)| {
          let local_state = global_user_state.clone();
          let sid = subscriber_ids.clone();
          let work = update_feed(feed_id, feed_url, subscriber_ids).and_then(move |new_items| {
            match new_items {
              Some(items) => {
                debug!("found {} new items for {}", items.len(), &feed_id);
                send_items(feed_id, items, &sid, &local_state);
              }
              None => (),
            };
            Ok(())
          });
          rt::spawn(work);
        },
      );
      Ok(())
    }).map_err(|e| panic!("delay errored; err={:?}", e));
  rt::spawn(update_subscriptions);
}

pub fn subscribe_feed(url: SubscribeParams, user_id: i32, state: UserWebsocketState) {
  let url = url.feed_url;
  debug!("subscribing: '{}' by '{}'", url, user_id);
  let work = db::get_feed_id(&url)
    .into_future()
    .and_then(|feed_id| {
      debug!("in db: '{}'", feed_id);
      Ok((feed_id, db::get_item_ids(&feed_id)))
    }).or_else(|_| {
      debug!("not in db: '{}'", url);
      add_feed(url)
    }).and_then(move |(feed_id, item_ids)| {
      db::subscribe_feed(&user_id, &feed_id);
      Ok((feed_id, item_ids))
    }).and_then(move |(feed_id, item_ids)| {
      match item_ids {
        Some(item_ids) => subscribe_new_items(&item_ids, &vec![user_id]),
        None => (),
      };
      Ok((feed_id, state))
    }).and_then(move |(feed_id, state)| {
      send_subscribeditems(feed_id, user_id, &state);
      Ok(())
    });
  rt::spawn(work);
}

pub fn add_feed(url: String) -> impl Future<Item = (i32, Option<Vec<i32>>), Error = ()> {
  fetch_feed(url.to_string())
    .and_then(|data| parse_fetched_data(&data))
    .and_then(move |data| handle_feed_types(data, &url))
    .and_then(|(new_feed, new_items)| {
      let new_ch = insert_channel(new_feed);
      Ok((new_items, new_ch.id))
    }).and_then(|(items, feed_id)| Ok((feed_id, handle_item_types(items, &feed_id))))
    .and_then(|(feed_id, items)| {
      let items = insert_items(&items).unwrap();
      let item_ids: Vec<_> = items.into_iter().map(|i| i.id).collect();
      Ok((feed_id, Some(item_ids)))
    })
}

pub fn update_feed(
  feed_id: i32,
  channel_url: String,
  subscriber_ids: Vec<i32>,
) -> impl Future<Item = Option<Vec<Item>>, Error = ()> {
  let local = channel_url.clone();
  fetch_feed(channel_url)
    .and_then(|data| parse_fetched_data(&data))
    .and_then(move |data| handle_feed_types(data, &local))
    .and_then(move |(_, items)| Ok(handle_item_types(items, &feed_id)))
    .and_then(|items| Ok(process_duplicates(items)))
    .and_then(move |new_items| match new_items {
      Some(items) => {
        let items = insert_items(&items).unwrap();
        let item_ids = items.iter().map(|i| i.id).collect();
        subscribe_new_items(&item_ids, &subscriber_ids);
        Ok(Some(items))
      }
      None => Ok(None),
    })
}

fn send_items(
  feed_id: i32,
  new_items: Vec<Item>,
  subscriber_ids: &Vec<i32>,
  state: &UserWebsocketState,
) {
  let composites: Vec<_> = new_items
    .into_iter()
    .map(|item| CompositeItem::from_item(&item))
    .collect();
  for uid in subscriber_ids.iter() {
    send_ws(feed_id, *uid, &composites, state);
  }
}

fn send_subscribeditems(feed_id: i32, user_id: i32, state: &UserWebsocketState) {
  let items = db::get_subscribed_items(feed_id, user_id, None);
  let composites: Vec<_> = items
    .unwrap()
    .into_iter()
    .map(|item| CompositeItem::from_subscribed(&item))
    .collect();
  send_ws(feed_id, user_id, &composites, state);
}

fn send_ws(
  feed_id: i32,
  user_id: i32,
  composites: &Vec<CompositeItem>,
  state: &UserWebsocketState,
) {
  let feed = db::get_subscribed_feed(&user_id, &feed_id);
  let msg = OutgoingWebsocketMessage::new_feed(feed.unwrap());
  ws_send_message(&user_id, msg.to_message(), &state);
  let msg = OutgoingWebsocketMessage::new_items(feed_id, composites.to_vec());
  ws_send_message(&user_id, msg.to_message(), &state);
}

/////////////////////////
/// Future components ///
/////////////////////////

pub fn fetch_feed(url: String) -> impl Future<Item = Vec<u8>, Error = ()> {
  let https = HttpsConnector::new(2).expect("TLS initialization failed");
  let client = Client::builder().build::<_, Body>(https);
  let local = url.to_owned();
  client
    .get(url.parse().unwrap())
    .map_err(move |err| error!("could not fetch: '{}': {}", url, err))
    .and_then(move |res| {
      debug!("fetching: '{}'", local);
      res
        .into_body()
        .concat2()
        .map_err(|_err| ())
        .and_then(move |body| {
          debug!("collected body: {}", local);
          Ok(body.to_vec())
        })
    })
}

///////////////////
/// Synchronous ///
///////////////////

fn parse_fetched_data(string: &[u8]) -> Result<FeedType, ()> {
  let mut buf = Vec::new();
  let mut reader = Reader::from_str(str::from_utf8(string).unwrap());
  loop {
    match reader.read_event(&mut buf) {
      Ok(Event::Start(ref e)) => match e.name() {
        b"rss" => {
          debug!("found rss");
          match rss::Channel::read_from(BufReader::new(string)) {
            Ok(channel) => return Ok(FeedType::RSS(channel)),
            Err(_) => return Err(()),
          }
        }
        b"feed" => {
          debug!("found atom");
          match atom_syndication::Feed::read_from(BufReader::new(string)) {
            Ok(feed) => return Ok(FeedType::Atom(feed)),
            Err(_) => return Err(()),
          }
        }
        _ => (),
      },
      _ => (),
    }
  }
}

fn handle_feed_types(parsed: FeedType, url: &str) -> Result<(NewFeed, ItemType), ()> {
  match parsed {
    FeedType::RSS(feed) => {
      let new_feed = NewFeed::from_rss(&feed, &url);
      let new_items = ItemType::Item(feed.items().to_vec());
      Ok((new_feed, new_items))
    }
    FeedType::Atom(feed) => {
      let new_feed = NewFeed::from_atom(&feed, &url);
      let new_items = ItemType::Entry(feed.entries().to_vec());
      Ok((new_feed, new_items))
    }
  }
}

fn handle_item_types(parsed: ItemType, feed_id: &i32) -> Vec<NewItem> {
  match parsed {
    ItemType::Item(i) => process_items(i, feed_id),
    ItemType::Entry(i) => process_entries(i, feed_id),
  }
}

fn subscribe_new_items(inserted_items: &Vec<i32>, subscribers: &Vec<i32>) {
  let insertables: Vec<(&i32, &i32, bool)> = subscribers
    .iter()
    .flat_map(|s| {
      inserted_items
        .iter()
        .map(move |i| (s, i, false))
        .collect::<Vec<(&i32, &i32, bool)>>()
    }).collect::<Vec<(&i32, &i32, bool)>>();
  insert_subscribed_items(insertables);
}

fn process_items<'a>(feed_items: Vec<rss::Item>, channel_id: &'a i32) -> Vec<NewItem> {
  let items: Vec<NewItem> = feed_items
    .iter()
    .map(|item| NewItem::from_item(item, *channel_id))
    .collect();
  items
}
fn process_entries<'a>(
  feed_items: Vec<atom_syndication::Entry>,
  channel_id: &'a i32,
) -> Vec<NewItem> {
  let items: Vec<NewItem> = feed_items
    .iter()
    .map(|entry| NewItem::from_entry(entry, *channel_id))
    .collect();
  items
}

fn process_duplicates(items: Vec<NewItem>) -> Option<Vec<NewItem>> {
  let new_items = match find_duplicates(items.iter().map(|x| x.guid.as_str()).collect()) {
    Some(dupes) => {
      let guids: Vec<&str> = dupes.iter().map(|x| x.1.as_str()).collect();
      let (new_items, mut duplicated_items): (Vec<NewItem>, Vec<NewItem>) = items
        .into_iter()
        .partition(|x| !guids.contains(&x.guid.as_str()));

      let updated_items: Vec<(i32, NewItem)> = duplicated_items
        .into_iter()
        .filter_map(|d| {
          let idx = dupes.iter().find(|(_, y, _)| y == &d.guid).unwrap();
          if d.published_at != idx.2 {
            Some((idx.0, d))
          } else {
            None
          }
        }).collect();
      debug!("found {} updated items", updated_items.len());
      updated_items
        .into_iter()
        .for_each(|(id, item)| update_item(id, item));
      new_items
    }
    None => items,
  };
  match new_items.is_empty() {
    false => Some(new_items),
    true => None,
  }
}
