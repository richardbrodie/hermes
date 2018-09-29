use std::collections::HashMap;
use warp::ws::Ws2;
use warp::{self, Filter, Rejection};

mod handlers;
mod jwt;
mod rest;
pub mod types;
pub mod ws;

use self::jwt::{authenticate, decode_jwt};
use self::rest::{serve_static, show_feeds, show_item, show_items, ASSET_PATH};
use self::types::{AccessToken, AssetFile, LoginParams, UserWebsocketState};
use self::ws::ws_created;

use models::Claims;

pub fn verify_token() -> impl warp::Filter<Extract = (Claims,), Error = Rejection> + Clone {
  warp::path::index()
    .and(warp::header::<String>("authorization"))
    .and_then(make_claim)
}

pub fn make_claim(token: String) -> Result<Claims, Rejection> {
  match decode_jwt(token) {
    Ok(claim) => Ok(claim),
    Err(_) => Err(warp::reject()),
  }
}

pub fn start_web(state: UserWebsocketState) {
  let state2 = state.clone();
  let jwt_auth =
    warp::query::<AccessToken>().and_then(|token: AccessToken| make_claim(token.access_token));

  let authenticate = warp::post2()
    .and(warp::path("authenticate"))
    .and(warp::path::index())
    .and(warp::body::json())
    .and_then(|payload: LoginParams| authenticate(payload));

  let assets = warp::get2()
    .and(warp::path::param::<AssetFile>())
    .and_then(|a: AssetFile| serve_static(a));

  let star = warp::get2()
    .and(warp::any())
    .and(warp::fs::file(format!("{}/index.html", ASSET_PATH)));

  // /api/feeds
  let api_feeds = warp::path("api")
    .and(warp::path("feeds"))
    .and(jwt_auth)
    .and_then(|claims| show_feeds(claims));
  // /api/item/:item_id
  let api_item = warp::path("api")
    .and(warp::path("item"))
    .and(warp::path::param::<i32>())
    .and(jwt_auth)
    .and_then(|item_id, claims| show_item(claims, item_id));
  // /api/items/:feed_id
  let api_items = warp::path("api")
    .and(warp::path("items"))
    .and(warp::path::param::<i32>())
    .and(warp::query::<HashMap<String, String>>())
    .and(jwt_auth)
    .and_then(|feed_id, query: HashMap<String, String>, claims| show_items(claims, feed_id, query));

  let ws = warp::path("ws")
    .and(warp::ws2())
    .and(jwt_auth)
    .map(move |ws: Ws2, claims: Claims| {
      let state = state2.clone();
      ws.on_upgrade(|websocket| ws_created(websocket, claims, state))
    });

  let api = api_feeds.or(api_items).or(api_item);
  let routes = authenticate.or(api).or(assets).or(ws).or(star);
  warp::serve(routes).run(([0, 0, 0, 0], 3030));
}
