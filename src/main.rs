#![allow(unused)]
extern crate atom_syndication;
extern crate base64;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate jsonwebtoken;
#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;
extern crate quick_xml;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate regex;
extern crate rss;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate sha2;
extern crate tokio;
extern crate tokio_fs;
extern crate tokio_io;
extern crate url;
extern crate warp;

use dotenv::dotenv;
use hyper::rt;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

pub mod db;
pub mod feed;
pub mod models;
pub mod schema;
pub mod views;
pub mod web;

use db::create_admin_user;
use feed::start_interval_loops;
use web::{start_web, types::UserWebsocketState};

fn main() {
  dotenv().ok();
  env::set_var("RUST_LOG", "hermes=info");
  pretty_env_logger::init();

  create_admin_user();

  rt::run(rt::lazy(|| {
    let state = Arc::new(Mutex::new(HashMap::new()));
    let global_user_state = UserWebsocketState { state: state };

    start_interval_loops(global_user_state.clone());
    start_web(global_user_state.clone());
    Ok(())
  }));
}
