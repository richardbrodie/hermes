use chrono::{DateTime, Utc};
use futures::Future;
use std::collections::HashMap;
use std::io;
use std::{path, str};
use tokio_fs;
use tokio_io;

use hyper::Body;
use warp::http::Response;
use warp::{self, Rejection};

use super::types::AssetFile;
use db::{get_subscribed_feeds, get_subscribed_item, get_subscribed_items};
use models::Claims;

pub static ASSET_PATH: &'static str = "./ui/dist/static";

/// feeds ///

pub fn show_feeds(claims: Claims) -> Result<impl warp::Reply, warp::Rejection> {
  match get_subscribed_feeds(&claims.id) {
    Some(feeds) => Ok(warp::reply::json(&feeds)),
    None => Err(warp::reject::not_found()),
  }
}

/// items ///

pub fn show_item(claims: Claims, item_id: i32) -> Result<impl warp::Reply, warp::Rejection> {
  let user_id = claims.id.clone();
  let got_item = get_subscribed_item(item_id, user_id);
  match got_item {
    Some(mut data) => {
      data.seen = true;
      Ok(warp::reply::json(&data))
    }
    None => Err(warp::reject::bad_request()),
  }
}

pub fn show_items(
  claims: Claims,
  feed_id: i32,
  query: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
  let updated = match query.get("updated") {
    Some(d) => match d.parse::<DateTime<Utc>>() {
      Ok(t) => Some(t),
      Err(_) => return Err(warp::reject()),
    },
    None => None,
  };

  match get_subscribed_items(feed_id, claims.id, updated) {
    Some(data) => Ok(warp::reply::json(&data)),
    None => Err(warp::reject::not_found()),
  }
}

/// assets ///

pub fn serve_static(
  asset: AssetFile,
) -> impl Future<Item = Response<Body>, Error = Rejection> + Send {
  let asset_path = path::Path::new(&ASSET_PATH).join(asset.0);
  tokio_fs::file::File::open(asset_path)
    .and_then(move |file| {
      let buf: Vec<u8> = Vec::new();
      tokio_io::io::read_to_end(file, buf)
        .and_then(|(_, b)| Ok(Response::builder().body(Body::from(b)).unwrap()))
    }).or_else(|e| {
      error!("file open error: {} ", e);
      let err = match e.kind() {
        io::ErrorKind::NotFound => warp::reject::not_found().with(e),
        _ => warp::reject::server_error().with(e),
      };
      Err(err)
    })
}
