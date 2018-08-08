use chrono::{DateTime, FixedOffset, NaiveDateTime};
use futures::{future, Future, Stream};
use hyper::header::{
  ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
  ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_EXPOSE_HEADERS, ALLOW, AUTHORIZATION,
};
use hyper::service::service_fn;
use hyper::{rt, Body, Error, Method, Request, Response, Server, StatusCode};
use jsonwebtoken::{decode, encode, Header, Validation};
use regex::Regex;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::{env, path, str};
use tokio_fs;
use tokio_io;
use url::form_urlencoded;

use db::{self, get_channels, get_item, get_items};
use feed;
use models::{Claims, User};
use router::Router;

static ASSET_PATH: &'static str = "react-ui/build";

pub fn router() -> Router {
  let mut router = Router::build();
  router
    .auth_handler(decode_jwt)
    .open_route(Method::OPTIONS, ".*", options_headers)
    .open_route(Method::GET, "/", home)
    .open_route(Method::GET, "/static/(.+)", show_asset)
    .open_route(Method::POST, "/authenticate", authenticate)
    .closed_route(Method::GET, "/feeds", index)
    // .closed_route(Method::GET, r"/feed/(\d+)", show_channel)
    .closed_route(Method::GET, r"/item/(\d+)", show_item)
    .closed_route(Method::GET, r"/items/(\d+|\d+\?.*)", show_items)
    .closed_route(Method::POST, "/add_feed", add_feed);
  router
}

pub fn start_web() {
  let addr = "127.0.0.1:4000".parse().unwrap();

  rt::spawn(future::lazy(move || {
    let service = move || {
      let router = router();
      service_fn(move |req| router.parse(req))
    };
    let server = Server::bind(&addr)
      .serve(service)
      .map_err(|e| eprintln!("server error: {}", e));

    info!("server running on {:?}", addr);
    server
  }));
}

fn add_feed(req: Request<Body>, claims: &Claims) -> ResponseFuture {
  let user_id = claims.id.clone();
  let response = req.into_body().concat2().map(move |chunk| {
    let raw_params_str = str::from_utf8(chunk.as_ref()).unwrap();
    let params: HashMap<String, String> = match serde_json::from_str(raw_params_str) {
      Ok(p) => p,
      Err(_) => form_urlencoded::parse(chunk.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>(),
    };

    let mut body = Body::empty();
    let mut status = StatusCode::BAD_REQUEST;

    match params.get("feed_url") {
      Some(n) => {
        if !n.is_empty() {
          info!("feed: {:?}", n);
          feed::subscribe_feed(n.to_owned(), user_id);
          status = StatusCode::OK;
        }
      }
      None => body = Body::from("parameter 'feed_url' missing"),
    }
    Response::builder()
      .header("Access-Control-Allow-Origin", "*")
      .status(status)
      .body(body)
      .unwrap()
  });
  Box::new(response)
}

fn home(_req: Request<Body>) -> ResponseFuture {
  let mut f = File::open(format!("{}/index.html", ASSET_PATH)).unwrap();
  let mut buffer = String::new();
  f.read_to_string(&mut buffer).unwrap();
  Router::response(Body::from(buffer), StatusCode::OK)
}

fn index(_req: Request<Body>, claims: &Claims) -> ResponseFuture {
  let user_id = claims.id.clone();
  let channels = db::get_subscribed_channels(&user_id);
  let mut body = Body::empty();
  let mut status = StatusCode::NOT_FOUND;
  match serde_json::to_string(&channels) {
    Ok(json) => {
      status = StatusCode::OK;
      body = Body::from(json);
    }
    Err(_) => (),
  };
  Router::response(body, status)
}

fn authenticate(req: Request<Body>) -> ResponseFuture {
  let response = req.into_body().concat2().map(move |chunk| {
    let mut status = StatusCode::UNAUTHORIZED;
    let mut body = Body::empty();

    let raw_params_str = str::from_utf8(chunk.as_ref()).unwrap();
    let params: HashMap<String, String> = serde_json::from_str(raw_params_str).unwrap();

    match (params.get("username"), params.get("password")) {
      (Some(u), Some(p)) => match User::check_user(&u, &p) {
        Some(user) => {
          status = StatusCode::OK;
          let jwt = generate_jwt(&user).unwrap();
          let json = json!({
            "token": jwt,
          });
          body = Body::from(json.to_string());
        }
        _ => (),
      },
      _ => status = StatusCode::BAD_REQUEST,
    };
    Response::builder()
      .header("Access-Control-Allow-Origin", "*")
      .status(status)
      .body(body)
      .unwrap()
  });
  Box::new(response)
}

// fn show_channel(req: Request<Body>, claims: &Claims) -> ResponseFuture {
//   let req_path = req.uri().path();
//   let re = Regex::new(r"/feed/(\d+)").unwrap();
//   let ch_id = match re.captures(req_path) {
//     Some(d) => d.get(1).unwrap().as_str(),
//     None => return Router::response(Body::empty(), StatusCode::NOT_FOUND),
//   };

//   let mut status = StatusCode::NOT_FOUND;
//   let mut body = Body::empty();
//   let data = get_items(ch_id.parse::<i32>().unwrap(), None);
//   match serde_json::to_string(&data) {
//     Ok(json) => {
//       body = Body::from(json);
//       status = StatusCode::OK;
//     }
//     Err(_) => (),
//   };
//   Router::response(body, status)
// }

fn show_item(req: Request<Body>, claims: &Claims) -> ResponseFuture {
  let req_path = req.uri().path();
  let re = Regex::new(r"/item/(\d+)").unwrap();
  let ch_id = match re.captures(req_path) {
    Some(d) => d.get(1).unwrap().as_str(),
    None => {
      info!("no match: {}", req_path);
      return Router::response(Body::empty(), StatusCode::BAD_REQUEST);
    }
  };

  let mut status = StatusCode::OK;
  let mut body = Body::empty();
  match get_item(ch_id.parse::<i32>().unwrap()) {
    Some(data) => match serde_json::to_string(&data) {
      Ok(json) => {
        body = Body::from(json);
        status = StatusCode::OK;
      }
      Err(_) => (),
    },
    None => (),
  };
  Router::response(body, status)
}

fn show_items(req: Request<Body>, claims: &Claims) -> ResponseFuture {
  let req_path = req.uri().path();
  let re = Regex::new(r"/items/(\d+)").unwrap();
  let ch_id = match re.captures(req_path) {
    Some(d) => d.get(1).unwrap().as_str(),
    None => {
      info!("no match: {}", req_path);
      return Box::new(future::ok(Response::new(Body::empty())));
    }
  };

  let mut body = Body::empty();
  let mut status = StatusCode::OK;

  let updated = match req.uri().query() {
    Some(s) => {
      let params = form_urlencoded::parse(s.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();
      match params.get("updated") {
        Some(d) => match NaiveDateTime::parse_from_str(d, "%Y-%m-%dT%H:%M:%S") {
          Ok(t) => Some(t),
          Err(err) => return Router::response(body, StatusCode::BAD_REQUEST),
        },
        None => None,
      }
    }
    None => None,
  };

  let data = get_items(ch_id.parse::<i32>().unwrap(), updated);
  match serde_json::to_string(&data) {
    Ok(json) => body = Body::from(json),
    Err(_) => status = StatusCode::NOT_FOUND,
  };
  Router::response(body, status)
}

fn show_asset(req: Request<Body>) -> ResponseFuture {
  let req_path = req.uri().path();
  let re = Regex::new(r"/static/(.+)").unwrap();
  let d = match re.captures(req_path) {
    Some(d) => d.get(1).unwrap().as_str(),
    None => {
      info!("no param match");
      return Router::response(Body::empty(), StatusCode::NOT_FOUND);
    }
  };

  let f = path::Path::new(&format!("{}/static", ASSET_PATH)).join(d);

  let response = tokio_fs::file::File::open(f)
    .and_then(|file| {
      let buf: Vec<u8> = Vec::new();
      tokio_io::io::read_to_end(file, buf)
        .and_then(|item| Ok(Response::new(item.1.into())))
        .or_else(|_| {
          Ok(
            Response::builder()
              .status(StatusCode::INTERNAL_SERVER_ERROR)
              .body(Body::empty())
              .unwrap(),
          )
        })
    })
    .or_else(|_| {
      info!("not found!");
      Ok(
        Response::builder()
          .status(StatusCode::NOT_FOUND)
          .body(Body::empty())
          .unwrap(),
      )
    });
  Box::new(response)
}

fn options_headers(_req: Request<Body>) -> ResponseFuture {
  Box::new(future::ok(
    Response::builder()
      .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
      .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
      .header(ACCESS_CONTROL_EXPOSE_HEADERS, "Access-Control-*")
      .header(
        ACCESS_CONTROL_ALLOW_HEADERS,
        "Access-Control-*, Origin, X-Requested-With, Content-Type, Accept, Authorization",
      )
      .header(
        ACCESS_CONTROL_ALLOW_METHODS,
        "GET, POST, PUT, PATCH, OPTIONS, HEAD",
      )
      .header(ALLOW, "GET, POST, PUT, PATCH, OPTIONS, HEAD")
      .body(Body::empty())
      .unwrap(),
  ))
}

fn decode_jwt(req: &Request<Body>) -> Option<Claims> {
  let secret = env::var("JWT_SECRET").unwrap();

  let r = r"^Bearer\s([\w_-]+\.[\w_-]+\.[\w=_-]+)$";
  let regex = Regex::new(&r).unwrap();
  let token = match req.headers().get(AUTHORIZATION) {
    Some(val) => match regex.captures(val.to_str().unwrap()) {
      Some(d) => d.get(1).unwrap().as_str(),
      None => return None,
    },
    None => return None,
  };

  match decode::<Claims>(&token, secret.as_ref(), &Validation::default()) {
    Ok(jwt) => Some(jwt.claims),
    Err(_) => None,
  }
}

fn generate_jwt(user: &User) -> Option<String> {
  let claims = Claims {
    name: user.username.to_string(),
    id: user.id,
  };

  match env::var("JWT_SECRET") {
    Ok(val) => match encode(&Header::default(), &claims, &val.as_ref()) {
      Ok(jwt) => Some(jwt),
      Err(_) => None,
    },
    Err(_) => None,
  }
}

// types

pub type AuthenticationHandler = for<'r> fn(&'r Request<Body>) -> Option<Claims>;
pub type ProtectedRequestFuture = for<'r> fn(Request<Body>, &'r Claims) -> ResponseFuture;
pub type UnprotectedRequestFuture = fn(Request<Body>) -> ResponseFuture;
pub type ResponseFuture = Box<Future<Item = Response<Body>, Error = Error> + Send + 'static>;

#[derive(Clone)]
pub enum RequestSignature {
  Closed(ProtectedRequestFuture),
  Open(UnprotectedRequestFuture),
}
