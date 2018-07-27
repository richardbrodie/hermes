#![allow(unused)]
#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate futures;
extern crate hyper;
extern crate jsonwebtoken;
#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate regex;
extern crate rss;
#[macro_use]
extern crate serde_json;
extern crate sodiumoxide;
extern crate tokio;
extern crate tokio_fs;
extern crate tokio_io;
extern crate url;
#[macro_use]
extern crate serde_derive;

pub mod db;
pub mod feed;
pub mod models;
pub mod router;
pub mod schema;
pub mod web;
