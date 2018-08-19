use chrono::{DateTime, FixedOffset, NaiveDateTime};
use diesel::dsl::exists;
use diesel::prelude::*;
use diesel::{self, select, PgConnection};
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use schema::{feeds, items, subscribed_feeds, subscribed_items, users};
use std::{env, thread};

use models::{CompositeItem, Feed, Item, NewFeed, NewItem, SubscribedFeed, SubscribedItem, User};

lazy_static! {
  static ref POOL: Pool<ConnectionManager<PgConnection>> = {
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

// seed admin user
pub fn create_admin_user() {
  use schema::users::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();

  match select(exists(users.filter(username.eq("admin")))).get_result::<bool>(&*connection) {
    Ok(true) => (),
    Ok(false) => {
      let admin_pass = env::var("ADMIN_PASS").expect("ADMIN_PASS must be set");
      let pwh = User::hash_pw(&admin_pass);
      diesel::insert_into(users)
        .values((username.eq("admin"), password_hash.eq(&pwh.as_bytes())))
        .load::<User>(&*connection)
        .expect("Error creating admin user");
    }
    Err(_) => error!("could not check if admin user existed"),
  }
}

// channels

pub fn find_channel_by_url(url: &str) -> Option<Feed> {
  use schema::feeds::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match feeds.filter(feed_link.eq(url)).first::<Feed>(&*connection) {
    Ok(ch) => Some(ch),
    Err(_) => None,
  }
}

pub fn get_channel_id(url: &str) -> Result<i32, diesel::result::Error> {
  use schema::feeds::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  feeds
    .filter(feed_link.eq(url))
    .select(id)
    .first(&*connection)
}

pub fn insert_channel(channel: NewFeed) -> Feed {
  let pool = establish_pool();
  let connection = pool.get().unwrap();

  diesel::insert_into(feeds::table)
    .values(&channel)
    .get_result::<Feed>(&*connection)
    .expect("Error saving new post")
}

// used during update loop
pub fn get_channel_urls_and_subscribers() -> Vec<(i32, String, Vec<i32>)> {
  use schema::feeds;
  use schema::subscribed_feeds;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  let channels = feeds::table.load::<Feed>(&*connection).unwrap();
  let subscribed_feeds = SubscribedFeed::belonging_to(&channels)
    .load::<SubscribedFeed>(&*connection)
    .unwrap()
    .grouped_by(&channels);
  let data = channels
    .into_iter()
    .zip(subscribed_feeds)
    .collect::<Vec<_>>();
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
  use schema::items;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match diesel::insert_into(items::table)
    .values(items)
    .get_results::<Item>(&*connection)
  {
    Ok(items) => Some(items.into_iter().map(|i| i.id).collect()),
    Err(_) => None,
  }
}

pub fn update_item(iid: i32, item: NewItem) {
  use schema::items::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  diesel::update(items.find(iid))
    .set((
      title.eq(item.title),
      link.eq(item.link),
      summary.eq(item.summary),
      published_at.eq(item.published_at),
      content.eq(item.content),
    ))
    .execute(&*connection);
}

pub fn find_duplicates(guids: Vec<&str>) -> Option<Vec<(i32, String, Option<NaiveDateTime>)>> {
  use schema::items::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  let results = items
    .filter(guid.eq_any(guids))
    .select((id, guid, published_at))
    .load(&*connection)
    .expect("Error loading items");
  match results.is_empty() {
    true => None,
    false => Some(results),
  }
}

pub fn get_item_ids(fid: &i32) -> Option<Vec<i32>> {
  use schema::items::dsl::*;
  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match items.filter(feed_id.eq(fid)).select(id).load(&*connection) {
    Ok(i) => Some(i),
    Err(_) => None,
  }
}

pub fn get_latest_item_date(feed_id: i32) -> Option<NaiveDateTime> {
  use schema::items::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match items
    .filter(feed_id.eq(feed_id))
    .order(published_at.desc())
    .first::<Item>(&*connection)
  {
    Ok(item) => item.published_at,
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

// subscribed_feeds

pub fn subscribe_channel(uid: &i32, fid: &i32) {
  use schema::subscribed_feeds::dsl::*;

  let pool = establish_pool();
  let connection = pool.get().unwrap();

  match diesel::insert_into(subscribed_feeds)
    .values((feed_id.eq(fid), user_id.eq(uid)))
    .execute(&*connection)
  {
    Ok(_) => info!("subscribed: '{}' by '{}'", fid, uid),
    Err(e) => error!("subscribe failure: '{}' by '{}': {}", fid, uid, e),
  };
}

pub fn get_subscribed_channels(uid: &i32) -> Option<Vec<Feed>> {
  use schema::feeds;
  use schema::subscribed_feeds;

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  match subscribed_feeds::table
    .inner_join(feeds::table)
    .filter(subscribed_feeds::user_id.eq(uid))
    .select((
      feeds::id,
      feeds::title,
      feeds::description,
      feeds::site_link,
      feeds::feed_link,
      feeds::updated_at,
    ))
    .load::<Feed>(&*connection)
  {
    Ok(feeds) => Some(feeds),
    Err(_) => None,
  }
}

pub fn get_subscribed_items(
  fid: i32,
  uid: i32,
  updated: Option<NaiveDateTime>,
) -> Option<Vec<CompositeItem>> {
  use schema::items;
  use schema::subscribed_items;

  let pool = establish_pool();
  let handle = thread::spawn(move || {
    let connection = pool.get().unwrap();
    let mut query = items::table
      .inner_join(subscribed_items::table)
      .filter(items::feed_id.eq(fid))
      .filter(subscribed_items::user_id.eq(uid))
      .order(items::published_at.desc())
      .into_boxed();
    if let Some(d) = updated {
      query = query.filter(items::published_at.lt(updated.unwrap()))
    }
    match query
      .limit(50)
      .select((
        items::id,
        items::title,
        items::summary,
        items::published_at,
        items::updated_at,
        subscribed_items::seen,
      ))
      .load::<(
        i32,
        String,
        Option<String>,
        Option<NaiveDateTime>,
        Option<NaiveDateTime>,
        bool,
      )>(&*connection)
    {
      Ok(items) => Some(
        items
          .into_iter()
          .map(|mut i| CompositeItem::partial(i))
          .collect(),
      ),
      Err(_) => None,
    }
  });
  handle.join().unwrap()
}

pub fn get_subscribed_item(iid: i32, uid: i32) -> Option<CompositeItem> {
  use schema::items;
  use schema::subscribed_items;

  let pool = establish_pool();
  let handle = thread::spawn(move || {
    let connection = pool.get().unwrap();

    let item = items::table.find(iid).first::<Item>(&*connection).unwrap();
    let subscribed = SubscribedItem::belonging_to(&item)
      .filter(subscribed_items::user_id.eq(uid))
      .first::<SubscribedItem>(&*connection);
    match subscribed {
      Ok(s) => {
        diesel::update(&s)
          .set(subscribed_items::seen.eq(true))
          .execute(&*connection);
        Some(CompositeItem {
          item_id: item.id,
          title: item.title,
          link: Some(item.link),
          summary: item.summary,
          published_at: item.published_at,
          updated_at: item.updated_at,
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
  use schema::subscribed_items;

  let insertables: Vec<_> = items
    .iter()
    .map(|i| {
      (
        subscribed_items::user_id.eq(i.0),
        subscribed_items::item_id.eq(i.1),
        subscribed_items::seen.eq(i.2),
      )
    })
    .collect();

  let pool = establish_pool();
  let connection = pool.get().unwrap();
  let inserted_items = diesel::insert_into(subscribed_items::table)
    .values(insertables)
    .execute(&*connection)
    .expect("Error saving new post");
}

//deprecated

// pub fn get_channel(id: i32) -> Option<Feed> {
//   let pool = establish_pool();
//   let connection = pool.get().unwrap();
//   match feeds.find(id).first::<Feed>(&*connection) {
//     Ok(feed) => Some(feed),
//     Err(_) => None,
//   }
// }

// pub fn get_channels() -> Vec<Feed> {
//   let pool = establish_pool();
//   let connection = pool.get().unwrap();
//   let results = feeds
//     .load::<Feed>(&*connection)
//     .expect("Error loading feeds");
//   results
// }

// pub fn get_item(id: i32) -> Option<Item> {
//   use schema::items::dsl::*;

//   let pool = establish_pool();
//   let connection = pool.get().unwrap();
//   match items.find(id).first::<Item>(&*connection) {
//     Ok(item) => Some(item),
//     Err(_) => None,
//   }
// }

// pub fn get_items(id: i32, updated: Option<NaiveDateTime>) -> Vec<Item> {
//   use schema::items::dsl::*;

//   let pool = establish_pool();
//   let handle = thread::spawn(move || {
//     let connection = pool.get().unwrap();
//     let mut query = items.filter(feed_channel_id.eq(id)).into_boxed();
//     if let Some(d) = updated {
//       query = query.filter(published_at.lt(updated.unwrap()))
//     }
//     query
//       .order(published_at.desc())
//       .limit(25)
//       .load::<Item>(&*connection)
//       .expect("Error loading feeds")
//   });
//   handle.join().unwrap()
// }

// pub fn mark_item_seen(iid: i32, uid: i32) {
//   use schema::subscribed_items;

//   let pool = establish_pool();
//   let handle = thread::spawn(move || {
//     let connection = pool.get().unwrap();

//     diesel::update(
//       subscribed_items::table
//         .filter(subscribed_items::feed_item_id.eq(iid))
//         .filter(subscribed_items::user_id.eq(uid)),
//     ).set(subscribed_items::seen.eq(true))
//       .execute(&*connection);
//   });
//   handle.join().unwrap()
// }
