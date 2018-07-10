use frank_jwt::{decode, encode, Algorithm};
use futures::{future, Future, Stream};
use hyper::header::{
  ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
  ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_EXPOSE_HEADERS, ALLOW, AUTHORIZATION,
};
use hyper::service::service_fn;
use hyper::{rt, Body, Error, HeaderMap, Method, Request, Response, Server, StatusCode};
use regex::Regex;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, io, path, str};
use tokio_fs;
use tokio_io;
use url::form_urlencoded;

use db::{get_channel_with_items, get_channels, get_item, get_items};
use feed;
use models::User;
use router::{RequestFuture, ResponseFuture, Router};

pub fn router() -> Router {
  let mut router = Router::build();
  router
    .auth_handler(check_jwt)
    .route(Method::OPTIONS, ".*", options_headers, false)
    .route(Method::GET, "/", home, false)
    .route(Method::GET, "/feeds", index, true)
    .route(Method::GET, "/static/(.+)", show_asset, false)
    .route(Method::GET, r"/feed/(\d+)", show_channel, true)
    .route(Method::GET, r"/item/(\d+)", show_item, true)
    .route(Method::GET, r"/items/(\d+)", show_items, true)
    .route(Method::POST, "/add_feed", add_feed, true)
    .route(Method::POST, "/authenticate", authenticate, false);
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

fn add_feed(req: Request<Body>) -> ResponseFuture {
  let response = req.into_body().concat2().map(move |chunk| {
    let params = form_urlencoded::parse(chunk.as_ref())
      .into_owned()
      .collect::<HashMap<String, String>>();

    match params.get("feed_url") {
      Some(n) => {
        info!("feed: {:?}", n);
        feed::add_feed(n.to_owned());
        Response::new(Body::empty())
      }
      None => Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("parameter 'feed_url' missing"))
        .unwrap(),
    }
  });
  Box::new(response)
}

fn home(req: Request<Body>) -> ResponseFuture {
  let mut f = File::open("vue/dist/index.html").unwrap();
  let mut buffer = String::new();
  f.read_to_string(&mut buffer).unwrap();
  Router::response(Body::from(buffer), StatusCode::OK)
}

fn index(req: Request<Body>) -> ResponseFuture {
  let channels = get_channels();
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

    let params = form_urlencoded::parse(chunk.as_ref())
      .into_owned()
      .collect::<HashMap<String, String>>();

    match (params.get("username"), params.get("password")) {
      (Some(u), Some(p)) => match User::check_user(&u, &p) {
        true => {
          status = StatusCode::OK;
          let jwt = generate_jwt(u).unwrap();
          body = Body::from(jwt);
        }
        _ => (),
      },
      _ => status = StatusCode::BAD_REQUEST,
    };
    Response::builder().status(status).body(body).unwrap()
  });
  Box::new(response)
}

fn show_channel(req: Request<Body>) -> ResponseFuture {
  let req_path = req.uri().path();
  let re = Regex::new(r"/feed/(\d+)").unwrap();
  let ch_id = match re.captures(req_path) {
    Some(d) => d.get(1).unwrap().as_str(),
    None => return Router::response(Body::empty(), StatusCode::NOT_FOUND),
  };

  let mut status = StatusCode::NOT_FOUND;
  let mut body = Body::empty();
  match get_channel_with_items(ch_id.parse::<i32>().unwrap()) {
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

fn show_item(req: Request<Body>) -> ResponseFuture {
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

fn show_items(req: Request<Body>) -> ResponseFuture {
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
  let data = get_items(ch_id.parse::<i32>().unwrap());
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

  let f = path::Path::new("vue/dist/static").join(d);

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
  let mut headers = HeaderMap::new();
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

fn check_jwt(req: &Request<Body>) -> bool {
  let map = req.headers();
  match map.get(AUTHORIZATION) {
    Some(_header) => true,
    None => false,
  }
}

fn generate_jwt(user: &str) -> Option<String> {
  let start = SystemTime::now();
  let since_the_epoch = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");

  let header = json!({
    "alg": "HS256",
    "typ": "JWT"
  });
  let payload = json!({
    "iat": since_the_epoch.as_secs(),
    "name": user,
  });

  match env::var("JWT_SECRET") {
    Ok(val) => match encode(header, &val.to_string(), &payload, Algorithm::HS256) {
      Ok(jwt) => Some(jwt),
      Err(_) => None,
    },
    Err(e) => None,
  }
}
