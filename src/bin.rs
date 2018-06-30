extern crate hyper;
extern crate pretty_env_logger;

extern crate feeds_lib;

use hyper::rt;
use std::env;

use feeds_lib::feed::start_feed_loop;
use feeds_lib::web::start_web;

fn main() {
  env::set_var("RUST_LOG", "feeds=debug");
  pretty_env_logger::init();

  rt::run(rt::lazy(|| {
    start_web();
    start_feed_loop();
    Ok(())
  }));
}
