use chrono::{DateTime, Utc};
use futures::stream::SplitSink;
use futures::{Future, Stream};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Header, Validation};
use regex::Regex;
use serde_json;
use std::collections::HashMap;
use std::io;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::{env, path, str};
use tokio_fs;
use tokio_io;

use hyper::Body;
use warp::http::{Response, StatusCode};
use warp::ws::{Message, WebSocket, Ws2};
use warp::{self, Filter, Rejection};

use db::{get_subscribed_feeds, get_subscribed_item, get_subscribed_items};
use feed;
use models::{Claims, User};

static ASSET_PATH: &'static str = "./ui/dist/static";

#[derive(Clone, Debug)]
pub struct UserWebsocketState {
  pub state: Arc<Mutex<HashMap<i32, SplitSink<WebSocket>>>>,
}
impl UserWebsocketState {
  pub fn clone(&self) -> Self {
    let s2 = Arc::clone(&self.state);
    UserWebsocketState { state: s2 }
  }
  pub fn insert(&self, key: i32, val: SplitSink<WebSocket>) {
    self.state.lock().unwrap().insert(key, val);
  }
  pub fn remove(&self, key: &i32) {
    self.state.lock().unwrap().remove(key);
  }
  // pub fn prep(&self) -> () {
  //   self.state.lock().unwrap();
  // }
}

#[derive(Deserialize, Debug)]
struct Login {
  username: String,
  password: String,
}

#[derive(Deserialize, Debug)]
struct AddFeed {
  feed_url: String,
}

#[derive(Deserialize, Debug)]
enum UserMessageType {
  MarkRead,
  Subscribe,
}

#[derive(Deserialize, Debug)]
struct UserMessage {
  msg_type: UserMessageType,
  data: String,
}
#[derive(Deserialize, Debug)]
struct AccessToken {
  access_token: String,
}

struct AssetFile(String);
impl FromStr for AssetFile {
  type Err = Rejection;
  fn from_str(s: &str) -> Result<AssetFile, Rejection> {
    let re = Regex::new(r"(src\.\w+\.(?:css|js))").unwrap();
    match re.captures(&s) {
      Some(m) => Ok(AssetFile(m.get(1).unwrap().as_str().to_owned())),
      None => Err(warp::reject::not_found()),
    }
  }
}

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
    .and_then(|payload: Login| authenticate(payload));

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
    .map(|claims| show_feeds(claims));
  // /api/item/:item_id
  let api_item = warp::path("api")
    .and(warp::path("item"))
    .and(warp::path::param::<i32>())
    .and(jwt_auth)
    .and_then(|item_id, claims| show_item(claims, item_id));
  // /api/add_feed
  let api_add_feed = warp::post2()
    .and(warp::path("api"))
    .and(warp::path("add_feed"))
    .and(jwt_auth)
    .and(warp::body::json())
    .and_then(move |claims: Claims, payload: AddFeed| {
      let state = state.clone();
      add_feed(claims.id, payload.feed_url, state)
    });
  // /api/items/:feed_id
  let api_items = warp::post2()
    .and(warp::path("api"))
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

  let api = api_feeds.or(api_items).or(api_item).or(api_add_feed);
  let routes = authenticate.or(api).or(assets).or(ws).or(star);
  warp::serve(routes).run(([127, 0, 0, 1], 3030));
}

fn ws_created(
  ws: WebSocket,
  claims: Claims,
  users: UserWebsocketState,
) -> impl Future<Item = (), Error = ()> {
  let user_id = claims.id;
  info!("user connected: {} - {}", user_id, claims.name);
  let (tx, rx) = ws.split();
  users.insert(user_id, tx);
  let users2 = users.clone();

  rx.for_each(move |msg| {
    user_incoming_msg(user_id, msg, &users);
    Ok(())
  }).then(move |result| {
      user_disconnected(&user_id, &users2);
      result
    })
    .map_err(move |e| {
      info!("websocket error(uid={}): {}", &user_id, e);
    })
}

fn user_incoming_msg(user_id: i32, msg: Message, users: &UserWebsocketState) {
  match serde_json::from_str::<UserMessage>(msg.to_str().unwrap()) {
    Ok(message) => match message.msg_type {
      UserMessageType::MarkRead => {
        info!("user {} read item {}", user_id, message.data);
      }
      UserMessageType::Subscribe => {
        debug!("user {} subscribed to {}", user_id, message.data);
        feed::subscribe_feed(message.data, user_id, users.to_owned());
      }
    },
    Err(_) => error!("Could not parse {:?} as a UserMessage", msg),
  };
}

fn user_disconnected(user_id: &i32, users: &UserWebsocketState) {
  info!("good bye user: {}", user_id);
  users.remove(user_id);
}

fn show_feeds(claims: Claims) -> impl warp::Reply {
  let channels = get_subscribed_feeds(&claims.id);
  warp::reply::json(&channels)
}

fn authenticate(params: Login) -> Result<impl warp::Reply, warp::Rejection> {
  match User::check_user(&params.username, &params.password) {
    Some(user) => {
      let jwt = generate_jwt(&user).unwrap();
      let json_body = json!({ "token": jwt, });
      Ok(warp::reply::json(&json_body))
    }
    _ => Err(warp::reject::bad_request()),
  }
}

fn show_item(claims: Claims, item_id: i32) -> Result<impl warp::Reply, warp::Rejection> {
  let user_id = claims.id.clone();
  let got_item = get_subscribed_item(item_id, user_id);
  match got_item {
    Some(data) => Ok(warp::reply::json(&data)),
    None => Err(warp::reject::bad_request()),
  }
}

fn show_items(
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

fn serve_static(asset: AssetFile) -> impl Future<Item = Response<Body>, Error = Rejection> + Send {
  let asset_path = path::Path::new(&ASSET_PATH).join(asset.0);
  tokio_fs::file::File::open(asset_path)
    .and_then(move |file| {
      let buf: Vec<u8> = Vec::new();
      tokio_io::io::read_to_end(file, buf)
        .and_then(|(_, b)| Ok(Response::builder().body(Body::from(b)).unwrap()))
    })
    .or_else(|e| {
      error!("file open error: {} ", e);
      let err = match e.kind() {
        io::ErrorKind::NotFound => warp::reject::not_found().with(e),
        _ => warp::reject::server_error().with(e),
      };
      Err(err)
    })
}

fn decode_jwt(token: String) -> Result<Claims, StatusCode> {
  let secret = env::var("JWT_SECRET").unwrap();

  // let r = r"^Bearer\s([\w_-]+\.[\w_-]+\.[\w=_-]+)$";
  // let regex = Regex::new(&r).unwrap();
  // let t = match regex.captures(&token) {
  //   Some(d) => d.get(1).unwrap().as_str(),
  //   None => return Err(StatusCode::UNAUTHORIZED),
  // };
  let t = token;

  let validation = Validation {
    validate_exp: false,
    ..Default::default()
  };
  let token = decode::<Claims>(&t, secret.as_ref(), &validation);
  match token {
    Ok(jwt) => {
      debug!("decoded: {:?}", jwt);
      Ok(jwt.claims)
    }
    Err(e) => {
      error!("failed to decode: {:?}", e);
      match *e.kind() {
        ErrorKind::ExpiredSignature => error!("expired: {:?}", e),
        ErrorKind::InvalidToken => error!("invalid: {:?}", e),
        _ => panic!(),
      }
      Err(StatusCode::UNAUTHORIZED)
    }
  }
}

fn generate_jwt(user: &User) -> Option<String> {
  let claims = Claims {
    name: user.username.to_string(),
    id: user.id,
  };

  match env::var("JWT_SECRET") {
    Ok(val) => {
      let token = encode(&Header::default(), &claims, &val.as_ref());
      match token {
        Ok(jwt) => {
          debug!("generated jwt: {:?}", jwt);
          Some(jwt)
        }
        Err(_) => None,
      }
    }
    Err(_) => None,
  }
}
