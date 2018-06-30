#![allow(unused)]
extern crate diesel;
extern crate feeds_lib;

use diesel::prelude::*;
use feeds_lib::db::establish_connection;
use feeds_lib::feed::add_feed;

fn main() {
  use feeds_lib::schema::users::dsl::*;
  let connection = establish_connection();
  diesel::delete(users)
    .execute(&connection)
    .expect("Error deleting users");

  diesel::insert_into(users)
    .values((username.eq("rbrodie"), password_hash.eq("apabepa")))
    .execute(&connection);

  // add_feed("http://feeds.arstechnica.com/arstechnica/index".to_string());
  // add_feed("http://feeds.bbci.co.uk/news/rss.xml".to_string());
}
