extern crate dotenv;
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate warp;

extern crate feeds_lib;

use dotenv::dotenv;
use hyper::rt;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use feeds_lib::db::create_admin_user;
use feeds_lib::feed::start_interval_loops;
use feeds_lib::web::{start_web, UserWebsocketState};

fn main() {
  dotenv().ok();
  env::set_var("RUST_LOG", "feeds=info");
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
