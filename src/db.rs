use chrono::NaiveDateTime;
use diesel::dsl::exists;
use diesel::prelude::*;
use diesel::{self, select, PgConnection};
use dotenv::dotenv;
use models::{FeedChannel, FeedItem, Subscription, User};
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use schema::feed_channels::dsl::*;
use schema::feed_items::dsl::*;
use schema::subscriptions::dsl::*;
use schema::users::dsl::*;
use schema::{feed_channels, feed_items, subscriptions, users};
use std::{env, thread};

#[derive(Insertable)]
#[table_name = "feed_channels"]
pub struct NewChannel<'a> {
  title: &'a str,
  site_link: &'a str,
  feed_link: &'a str,
  description: &'a str,
  updated_at: &'a NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "feed_items"]
pub struct NewItem<'a> {
  pub guid: &'a str,
  pub title: &'a str,
  pub link: &'a str,
  pub description: &'a str,
  pub published_at: NaiveDateTime,
  pub feed_channel_id: &'a i32,
  pub content: Option<&'a str>,
}
impl<'a> NewItem<'a> {
  pub fn new(item: &FeedItem) -> NewItem {
    NewItem {
      guid: &item.guid,
      title: &item.title,
      link: &item.link,
      description: &item.description,
      published_at: item.published_at,
      feed_channel_id: &item.feed_channel_id,
      content: match &item.content {
        Some(s) => Some(s),
        None => None,
      },
    }
  }
}

// internal

lazy_static! {
  static ref POOL: Pool<ConnectionManager<PgConnection>> = {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
      .build(manager)
      .expect("Failed to create pool.")
  };
}

pub fn establish_pool() -> Pool<ConnectionManager<PgConnection>> {
  POOL.clone()
}

// pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
//   dotenv().ok();

//   let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//   let manager = ConnectionManager::<PgConnection>::new(database_url);
//   Pool::builder()
//     .build(manager)
//     .expect("Failed to create pool.")
// }

// channels

//deprecated
pub fn get_channel(id: i32) -> Option<FeedChannel> {
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match feed_channels.find(id).first::<FeedChannel>(&*connection) {
    Ok(feed) => Some(feed),
    Err(_) => None,
  }
}

pub fn find_channel_by_url(url: &str) -> Option<FeedChannel> {
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match feed_channels
    .filter(feed_link.eq(url))
    .first::<FeedChannel>(&*connection)
  {
    Ok(ch) => Some(ch),
    Err(_) => None,
  }
}

pub fn get_channel_id(url: &str) -> Result<i32, diesel::result::Error> {
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  feed_channels
    .filter(feed_link.eq(url))
    .select(feed_channels::id)
    .first(&*connection)
}

pub fn insert_channel(channel: &mut FeedChannel) {
  let pool = establish_pool();
  let connection = pool.get().unwrap();

  let new_post = NewChannel {
    title: &channel.title,
    site_link: &channel.site_link,
    feed_link: &channel.feed_link,
    description: &channel.description,
    updated_at: &channel.updated_at,
  };

  let result = diesel::insert_into(feed_channels::table)
    .values(&new_post)
    .get_result::<FeedChannel>(&*connection)
    .expect("Error saving new post");

  channel.id = result.id;
}

// deprecated
pub fn get_channels() -> Vec<FeedChannel> {
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  let results = feed_channels
    .load::<FeedChannel>(&*connection)
    .expect("Error loading feeds");
  results
}

pub fn get_channel_urls() -> Vec<(i32, String)> {
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  let results = feed_channels
    .select((feed_channels::id, feed_channels::feed_link))
    .load(&*connection)
    .expect("Error loading feeds");
  results
}

//items

pub fn get_item(id: i32) -> Option<FeedItem> {
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match feed_items.find(id).first::<FeedItem>(&*connection) {
    Ok(item) => Some(item),
    Err(_) => None,
  }
}

pub fn get_items(id: i32) -> Vec<FeedItem> {
  let pool = establish_pool();
  let handle = thread::spawn(move || {
    let connection = pool.get().unwrap();
    feed_items
      .filter(feed_items::feed_channel_id.eq(id))
      .order(feed_items::published_at.desc())
      .limit(5)
      .load::<FeedItem>(&*connection)
      .expect("Error loading feeds")
  });
  handle.join().unwrap()
}

pub fn insert_items(items: &Vec<NewItem>) {
  use schema::feed_items;
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  diesel::insert_into(feed_items::table)
    .values(items)
    .execute(&*connection)
    .expect("Error saving new post");
}

pub fn update_item(id: i32, item: &NewItem) {
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  diesel::update(feed_items.find(id))
    .set(item)
    .execute(&*connection)
    .expect("Error updating item");
}

pub fn find_duplicates(guids: Vec<&str>) -> Option<Vec<(i32, String, NaiveDateTime)>> {
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  let results = feed_items
    .filter(guid.eq_any(guids))
    .select((feed_items::id, feed_items::guid, feed_items::published_at))
    .load(&*connection)
    .expect("Error loading items");
  match results.len() {
    0 => None,
    _ => Some(results),
  }
}

pub fn get_latest_item_date(channel_id: i32) -> Option<NaiveDateTime> {
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match feed_items
    .filter(feed_items::feed_channel_id.eq(channel_id))
    .order(feed_items::published_at.desc())
    .first::<FeedItem>(&*connection)
  {
    Ok(item) => Some(item.published_at),
    Err(_) => None,
  }
}

// users

pub fn get_user(uname: &str) -> Option<User> {
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match users.filter(username.eq(uname)).first::<User>(&*connection) {
    Ok(user) => Some(user),
    Err(_) => None,
  }
}

// subscriptions

pub fn subscribe(uid: &i32, fid: &i32) {
  let pool = establish_pool();
  let connection = pool.get().unwrap();

  match diesel::insert_into(subscriptions)
    .values((subscriptions::feed_channel_id.eq(fid), user_id.eq(uid)))
    .execute(&*connection)
  {
    Ok(r) => info!("res: {:?}", r),
    Err(_) => (),
  }
}

pub fn get_subscribed_channels(uid: &i32) -> Option<Vec<FeedChannel>> {
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match subscriptions
    .inner_join(feed_channels)
    .filter(subscriptions::user_id.eq(uid))
    .select((
      feed_channels::id,
      feed_channels::title,
      site_link,
      feed_link,
      feed_channels::description,
      updated_at,
    ))
    .load::<FeedChannel>(&*connection)
  {
    Ok(feeds) => Some(feeds),
    Err(_) => None,
  }
}
