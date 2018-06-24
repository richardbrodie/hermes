#![allow(unused)]
#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate futures;
extern crate hyper;
// extern crate num_cpus;
extern crate pretty_env_logger;
extern crate regex;
extern crate rss;
extern crate serde_json;
extern crate tokio;
extern crate tokio_fs;
extern crate tokio_io;
extern crate url;
#[macro_use]
extern crate serde_derive;

use hyper::rt;
use std::env;

mod db;
mod feed;
mod models;
mod schema;
mod web;

use feed::start_feed_loop;
use web::start_web;

fn main() {
  env::set_var("RUST_LOG", "feeds=debug");
  pretty_env_logger::init();

  rt::run(rt::lazy(|| {
    start_web();
    start_feed_loop();
    Ok(())
  }));
}
