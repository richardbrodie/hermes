use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::{FeedChannel, FeedItem};
use schema::feed_channels::dsl::*;
use schema::feed_items::dsl::*;
use schema::{feed_channels, feed_items};

#[derive(Insertable)]
#[table_name = "feed_channels"]
pub struct NewChannel<'a> {
  title: &'a str,
  link: &'a str,
  description: &'a str,
  updated_at: &'a NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "feed_items"]
pub struct NewItem<'a> {
  guid: &'a str,
  title: &'a str,
  link: &'a str,
  description: &'a str,
  published_at: &'a NaiveDateTime,
  feed_channel_id: &'a i32,
}

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn insert_items(items: &Vec<FeedItem>) {
  use schema::feed_items;
  let connection = establish_connection();

  let new_items: Vec<NewItem> = items
    .iter()
    .map(|item| NewItem {
      guid: &item.guid,
      title: &item.title,
      link: &item.link,
      description: &item.description,
      published_at: &item.published_at,
      feed_channel_id: &item.feed_channel_id,
    })
    .collect();

  diesel::insert_into(feed_items::table)
    .values(&new_items)
    .execute(&connection)
    .expect("Error saving new post");
}

pub fn insert_channel(channel: &mut FeedChannel) {
  let connection = establish_connection();

  let new_post = NewChannel {
    title: &channel.title,
    link: &channel.link,
    description: &channel.description,
    updated_at: &channel.updated_at,
  };

  let result = diesel::insert_into(feed_channels::table)
    .values(&new_post)
    .get_result::<FeedChannel>(&connection)
    .expect("Error saving new post");
  channel.id = result.id;
}

pub fn get_channel(id: i32) -> Option<FeedChannel> {
  let connection = establish_connection();
  match feed_channels.find(id).first::<FeedChannel>(&connection) {
    Ok(feed) => Some(feed),
    Err(_) => {
      // log error
      None
    }
  }
}

pub fn get_channel_with_items(id: i32) -> Option<(FeedChannel, Vec<FeedItem>)> {
  let connection = establish_connection();
  let res = get_channel(id);
  match res {
    Some(channel) => {
      let items = FeedItem::belonging_to(&channel)
        .order(feed_items::published_at.desc())
        .load::<FeedItem>(&connection)
        .expect("Error loading feeds");
      Some((channel, items))
    }
    None => None,
  }
}

pub fn get_channels() -> Vec<FeedChannel> {
  let connection = establish_connection();
  let results = feed_channels
    .limit(5)
    .load::<FeedChannel>(&connection)
    .expect("Error loading feeds");
  results
}
