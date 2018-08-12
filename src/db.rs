use chrono::{DateTime, FixedOffset, NaiveDateTime};
use diesel::dsl::exists;
use diesel::prelude::*;
use diesel::{self, select, PgConnection};
use dotenv::dotenv;
use models::{CompositeFeedItem, FeedChannel, FeedItem, SubscribedFeedItem, Subscription, User};
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
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

#[derive(Insertable, AsChangeset, Debug)]
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

    let pg_user = env::var("PG_USER").expect("PG_USER must be set");
    let pg_pass = env::var("PG_PASS").expect("PG_PASS must be set");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let pg_db = env::var("PG_DB").expect("PG_DB must be set");
    let database_url = format!("postgres://{}:{}@{}/{}", pg_user, pg_pass, db_host, pg_db);
    info!("database url: {}", database_url);

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
      .build(manager)
      .expect("Failed to create pool.")
  };
}

pub fn establish_pool() -> Pool<ConnectionManager<PgConnection>> {
  POOL.clone()
}

// channels

pub fn find_channel_by_url(url: &str) -> Option<FeedChannel> {
  use schema::feed_channels::dsl::*;

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
  use schema::feed_channels::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  feed_channels
    .filter(feed_link.eq(url))
    .select(id)
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

// used during update loop
pub fn get_channel_urls_and_subscribers() -> Vec<(i32, String, Vec<i32>)> {
  use schema::feed_channels;
  use schema::subscriptions;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  let channels = feed_channels::table
    .load::<FeedChannel>(&*connection)
    .unwrap();
  let subscriptions = Subscription::belonging_to(&channels)
    .load::<Subscription>(&*connection)
    .unwrap()
    .grouped_by(&channels);
  let data = channels.into_iter().zip(subscriptions).collect::<Vec<_>>();
  let result = data
    .into_iter()
    .map(|(f, s)| {
      (
        f.id,
        f.feed_link,
        s.into_iter().map(|i| i.user_id).collect(),
      )
    })
    .collect();
  result
}

//items

pub fn insert_items(items: &Vec<NewItem>) -> Option<Vec<i32>> {
  use schema::feed_items;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match diesel::insert_into(feed_items::table)
    .values(items)
    .get_results::<FeedItem>(&*connection)
  {
    Ok(items) => Some(items.into_iter().map(|i| i.id).collect()),
    Err(_) => None,
  }
}

pub fn update_item(iid: i32, item: &NewItem) {
  use schema::feed_items::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  diesel::update(feed_items.find(iid))
    .set((
      title.eq(item.title),
      link.eq(item.link),
      description.eq(item.description),
      published_at.eq(item.published_at),
      content.eq(item.content),
    ))
    .execute(&*connection)
    .expect(&format!("Error updating item {} with {:?}", iid, item));
}

pub fn find_duplicates(guids: Vec<&str>) -> Option<Vec<(i32, String, NaiveDateTime)>> {
  use schema::feed_items::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  let results = feed_items
    .filter(guid.eq_any(guids))
    .select((id, guid, published_at))
    .load(&*connection)
    .expect("Error loading items");
  match results.len() {
    0 => None,
    _ => Some(results),
  }
}

pub fn get_item_ids(fid: &i32) -> Option<Vec<i32>> {
  use schema::feed_items::dsl::*;
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match feed_items
    .filter(feed_channel_id.eq(fid))
    .select(id)
    .load(&*connection)
  {
    Ok(i) => Some(i),
    Err(_) => None,
  }
}

pub fn get_latest_item_date(channel_id: i32) -> Option<NaiveDateTime> {
  use schema::feed_items::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match feed_items
    .filter(feed_channel_id.eq(channel_id))
    .order(published_at.desc())
    .first::<FeedItem>(&*connection)
  {
    Ok(item) => Some(item.published_at),
    Err(_) => None,
  }
}

// users

pub fn get_user(uname: &str) -> Option<User> {
  use schema::users::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match users.filter(username.eq(uname)).first::<User>(&*connection) {
    Ok(user) => Some(user),
    Err(_) => None,
  }
}

// subscriptions

pub fn subscribe_channel(uid: &i32, fid: &i32) {
  use schema::subscriptions::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();

  diesel::insert_into(subscriptions)
    .values((feed_channel_id.eq(fid), user_id.eq(uid)))
    .execute(&*connection)
    .expect("Error subscribing");
}

pub fn get_subscribed_channels(uid: &i32) -> Option<Vec<FeedChannel>> {
  use schema::feed_channels;
  use schema::subscriptions;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match subscriptions::table
    .inner_join(feed_channels::table)
    .filter(subscriptions::user_id.eq(uid))
    .select((
      feed_channels::id,
      feed_channels::title,
      feed_channels::site_link,
      feed_channels::feed_link,
      feed_channels::description,
      feed_channels::updated_at,
    ))
    .load::<FeedChannel>(&*connection)
  {
    Ok(feeds) => Some(feeds),
    Err(_) => None,
  }
}

pub fn get_subscribed_items(
  fid: i32,
  uid: i32,
  updated: Option<NaiveDateTime>,
) -> Option<Vec<CompositeFeedItem>> {
  use schema::feed_items;
  use schema::subscribed_feed_items;

  let pool = establish_pool();
  let handle = thread::spawn(move || {
    let connection = pool.get().unwrap();
    let mut query = feed_items::table
      .inner_join(subscribed_feed_items::table)
      .filter(feed_items::feed_channel_id.eq(fid))
      .filter(subscribed_feed_items::user_id.eq(uid))
      .order(feed_items::published_at.desc())
      .into_boxed();
    if let Some(d) = updated {
      query = query.filter(feed_items::published_at.lt(updated.unwrap()))
    }
    match query
      .limit(50)
      .select((
        feed_items::id,
        feed_items::title,
        feed_items::description,
        feed_items::published_at,
        subscribed_feed_items::seen,
      ))
      .load::<(i32, String, String, NaiveDateTime, bool)>(&*connection)
    {
      Ok(items) => Some(
        items
          .iter()
          .map(|i| CompositeFeedItem::partial(i))
          .collect(),
      ),
      Err(_) => None,
    }
  });
  handle.join().unwrap()
}

pub fn get_subscribed_item(iid: i32, uid: i32) -> Option<CompositeFeedItem> {
  use schema::feed_items;
  use schema::subscribed_feed_items;

  let pool = establish_pool();
  let handle = thread::spawn(move || {
    let connection = pool.get().unwrap();

    let item = feed_items::table
      .find(iid)
      .first::<FeedItem>(&*connection)
      .unwrap();
    let subscribed = SubscribedFeedItem::belonging_to(&item)
      .filter(subscribed_feed_items::user_id.eq(uid))
      .first::<SubscribedFeedItem>(&*connection);
    match subscribed {
      Ok(s) => {
        diesel::update(&s)
          .set(subscribed_feed_items::seen.eq(true))
          .execute(&*connection);
        Some(CompositeFeedItem {
          item_id: item.id,
          title: item.title,
          link: Some(item.link),
          description: item.description,
          published_at: item.published_at,
          content: item.content,
          seen: true,
        })
      }
      Err(_) => None,
    }
  });
  handle.join().unwrap()
}

pub fn insert_subscribed_items(items: Vec<(&i32, &i32, bool)>) {
  use schema::subscribed_feed_items;

  let insertables: Vec<_> = items
    .iter()
    .map(|i| {
      (
        subscribed_feed_items::user_id.eq(i.0),
        subscribed_feed_items::feed_item_id.eq(i.1),
        subscribed_feed_items::seen.eq(i.2),
      )
    })
    .collect();

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  let inserted_items = diesel::insert_into(subscribed_feed_items::table)
    .values(insertables)
    .execute(&*connection)
    .expect("Error saving new post");
}

//deprecated

// pub fn get_channel(id: i32) -> Option<FeedChannel> {
//   let pool = establish_pool();
//   let connection = pool.get().unwrap();
//   match feed_channels.find(id).first::<FeedChannel>(&*connection) {
//     Ok(feed) => Some(feed),
//     Err(_) => None,
//   }
// }

// pub fn get_channels() -> Vec<FeedChannel> {
//   let pool = establish_pool();
//   let connection = pool.get().unwrap();
//   let results = feed_channels
//     .load::<FeedChannel>(&*connection)
//     .expect("Error loading feeds");
//   results
// }

// pub fn get_item(id: i32) -> Option<FeedItem> {
//   use schema::feed_items::dsl::*;

//   let pool = establish_pool();
//   let connection = pool.get().unwrap();
//   match feed_items.find(id).first::<FeedItem>(&*connection) {
//     Ok(item) => Some(item),
//     Err(_) => None,
//   }
// }

// pub fn get_items(id: i32, updated: Option<NaiveDateTime>) -> Vec<FeedItem> {
//   use schema::feed_items::dsl::*;

//   let pool = establish_pool();
//   let handle = thread::spawn(move || {
//     let connection = pool.get().unwrap();
//     let mut query = feed_items.filter(feed_channel_id.eq(id)).into_boxed();
//     if let Some(d) = updated {
//       query = query.filter(published_at.lt(updated.unwrap()))
//     }
//     query
//       .order(published_at.desc())
//       .limit(25)
//       .load::<FeedItem>(&*connection)
//       .expect("Error loading feeds")
//   });
//   handle.join().unwrap()
// }

// pub fn mark_item_seen(iid: i32, uid: i32) {
//   use schema::subscribed_feed_items;

//   let pool = establish_pool();
//   let handle = thread::spawn(move || {
//     let connection = pool.get().unwrap();

//     diesel::update(
//       subscribed_feed_items::table
//         .filter(subscribed_feed_items::feed_item_id.eq(iid))
//         .filter(subscribed_feed_items::user_id.eq(uid)),
//     ).set(subscribed_feed_items::seen.eq(true))
//       .execute(&*connection);
//   });
//   handle.join().unwrap()
// }
