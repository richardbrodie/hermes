#![allow(unused)]
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate askama;
extern crate dotenv;
extern crate env_logger;
extern crate reqwest;
extern crate rss;

use std::env;

mod db;
mod feed;
mod models;
mod schema;
mod template;
mod web;

use db::{get_channels, insert_channel};
use feed::{add_feed, fetch_feed};
use template::{BaseTemplate, FeedChannelTemplate};
use web::start_web;

fn main() {
  env::set_var("RUST_LOG", "actix_web=debug");
  env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();

  // let url = "http://lorem-rss.herokuapp.com/feed";
  // let url = "https://www.anandtech.com/rss/";
  let url = "http://feeds.arstechnica.com/arstechnica/index";
  // add_feed(&url);
  // get_channels();
  // let feed = FeedChannelTemplate::new(&channel);
  println!("{:#?}", fetch_feed(&url));
  // start_web();

  // run_hyper();
  // select_channels();
  // select_items();
}

// fn run_hyper() -> Result<(), hyper::Error> {
//   let mut core = Core::new()?;
//   let client = Client::new(&core.handle());

//   let uri = "http://httpbin.org/ip".parse()?;
//   let work = client.get(uri).and_then(|res| {
//     println!("Response: {}", res.status());

//     let b = res.body();
//     b.for_each(|chunk| io::stdout().write_all(&chunk).map_err(From::from))
//   });
//   core.run(work)?;
//   Ok(())
// }

// fn run_reqwest() -> Result<(), reqwest::Error> {
//   let body = reqwest::get("http://lorem-rss.herokuapp.com/feed")?;
//   let channel = Channel::read_from(BufReader::new(body)).unwrap();

//   println!("body = {:?}", channel);
//   Ok(())
// }
