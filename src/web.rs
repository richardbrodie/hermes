use chrono::{DateTime, Utc};
use futures::{future, Future, Stream};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Header, Validation};
use regex::Regex;
// use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::{env, path, str};
use tokio_fs;
use tokio_io;

use warp::filters::BoxedFilter;
use warp::http::{Response, StatusCode};
use warp::{self, Filter, Rejection, Reply};

use db::{get_subscribed_feeds, get_subscribed_item, get_subscribed_items};
use feed;
use models::{Claims, User};

static ASSET_PATH: &'static str = "./ui/dist/static";

#[derive(Deserialize, Debug)]
struct Login {
  username: String,
  password: String,
}

#[derive(Deserialize, Debug)]
struct AddFeed {
  feed_url: String,
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
  warp::header::<String>("authorization").and_then(|token| match decode_jwt(token) {
    Ok(claim) => Ok(claim),
    Err(code) => Err(warp::reject()),
  })
}

// pub fn serve_asset(
//   p: impl From<PathBuf>,
// ) -> impl warp::Filter<Extract = (File,), Error = Rejection> + Clone {
//   Ok(warp::fs::file(p))
// }

pub fn start_web() {
  let authenticate = warp::post2()
    .and(warp::path("authenticate"))
    .and(warp::path::index())
    .and(warp::body::json())
    .and_then(|payload: Login| authenticate(payload));

  let assets = warp::get2()
    .and(warp::path::param::<AssetFile>())
    .map(|asset: AssetFile| {
      path::Path::new(&ASSET_PATH)
        .join(asset.0)
        .to_str()
        .unwrap()
        .to_owned()
    });
  // .map(|asset: AssetFile| asset.0)
  // .and(warp::fs::dir(""))
  // .and(warp::fs::dir(ASSET_PATH))
  // .map(|_, p| {info!("{:?}",p); p});

  let star = warp::get2().and(warp::any()).map(|| index());

  // /api/feeds
  let api_feeds = warp::path("api")
    .and(warp::path("feeds"))
    .and(warp::path::index())
    .and(verify_token())
    .map(|claims| show_feeds(claims));
  // /api/item/:item_id
  let api_item = warp::path("api")
    .and(warp::path("item"))
    .and(warp::path::param::<i32>())
    .and(warp::path::index())
    .and(verify_token())
    .and_then(|item_id, claims| show_item(claims, item_id));
  // /api/add_feed
  let api_add_feed = warp::post2()
    .and(warp::path("api"))
    .and(warp::path("add_feed"))
    .and(warp::path::index())
    .and(verify_token())
    .and(warp::body::json())
    .and_then(|claims, payload: AddFeed| add_feed(claims, payload));
  // /api/items/:feed_id
  let api_items = warp::post2()
    .and(warp::path("api"))
    .and(warp::path("items"))
    .and(warp::path::param::<i32>())
    .and(warp::query::<HashMap<String, String>>())
    .and(warp::path::index())
    .and(verify_token())
    .and_then(|feed_id, query: HashMap<String, String>, claims| show_items(claims, feed_id, query));

  // let api = api_feeds.or(api_items).or(api_item).or(api_add_feed);
  // let routes = assets.or(authenticate).or(api).or(star);

  let routes = assets;
  warp::serve(routes).run(([127, 0, 0, 1], 3030));
}

fn add_feed(claims: Claims, payload: AddFeed) -> Result<impl warp::Reply, warp::Rejection> {
  let u = payload.feed_url;
  match u.is_empty() {
    false => {
      debug!("feed_url: {}", u);
      feed::subscribe_feed(u.to_owned(), claims.id);
      Ok(warp::reply())
    }
    true => Err(warp::reject()),
  }
}

fn index() -> impl warp::Reply {
  let mut f = File::open(format!("{}/index.html", ASSET_PATH)).unwrap();
  let mut buffer = String::new();
  f.read_to_string(&mut buffer).unwrap();
  Response::builder().body(buffer)
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
  let mut status = StatusCode::OK;
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

fn serve_static(asset: AssetFile) -> Result<impl warp::Reply, warp::Rejection> {
  let asset_path = path::Path::new(&ASSET_PATH).join(asset.0);
  warp::fs::file(asset_path)

let response = tokio_fs::file::File::open(asset_path)
  .and_then(move |file| {
    let buf: Vec<u8> = Vec::new();
    tokio_io::io::read_to_end(file, buf)
      .and_then(|item| Ok(warp::reply(item.1.into())))
      .or_else(|_| Err(warp::reject::server_error()))
  })
  .or_else(|e| {
    warn!("not found! - {}", e);
    Ok(
      Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap(),
    )
  });
Box::new(response)
}

fn decode_jwt(token: String) -> Result<Claims, StatusCode> {
  let secret = env::var("JWT_SECRET").unwrap();

  let r = r"^Bearer\s([\w_-]+\.[\w_-]+\.[\w=_-]+)$";
  let regex = Regex::new(&r).unwrap();
  let t = match regex.captures(&token) {
    Some(d) => d.get(1).unwrap().as_str(),
    None => return Err(StatusCode::UNAUTHORIZED),
  };

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
