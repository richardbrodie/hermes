extern crate dotenv;
extern crate hyper;
extern crate pretty_env_logger;

extern crate feeds_lib;

use dotenv::dotenv;
use hyper::rt;
use std::env;

use feeds_lib::db::create_admin_user;
use feeds_lib::feed::start_feed_loop;
use feeds_lib::web::start_web;

fn main() {
  dotenv().ok();
  env::set_var("RUST_LOG", "feeds=info");
  pretty_env_logger::init();

  create_admin_user();

  rt::run(rt::lazy(|| {
    start_web();
    start_feed_loop();
    Ok(())
  }));
}
