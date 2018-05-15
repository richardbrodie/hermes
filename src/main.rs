// #![allow(unused)]
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate askama;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate rss;
extern crate tokio_core;

use std::env;

mod db;
mod feed;
mod models;
mod schema;
mod template;
mod web;

// use db::{get_channels, insert_channel};
use feed::add_feed;
// use template::{BaseTemplate, FeedChannelTemplate};
use web::start_web;

fn main() {
  env::set_var("RUST_LOG", "actix_web=debug");
  env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();

  // let url = "http://lorem-rss.herokuapp.com/feed";
  // let url = "https://www.anandtech.com/rss/";
  let url = "http://feeds.arstechnica.com/arstechnica/index";
  // add_feed(&url);
  start_web();
}
