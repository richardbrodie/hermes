use futures::{future, Future, Stream};
use hyper::header::{
  ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
  ACCESS_CONTROL_EXPOSE_HEADERS, ALLOW,
};
use hyper::rt;
use hyper::service::service_fn;
use hyper::{Body, Error, HeaderMap, Method, Request, Response, Server, StatusCode};
use regex::Regex;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::{path, str};
use tokio_fs;
use tokio_io;
use url::form_urlencoded;

use db::{get_channel_with_items, get_channels, get_item, get_items};
use feed;

pub type ResponseFuture = Box<Future<Item = Response<Body>, Error = Error> + Send>;

pub fn start_web() {
  let addr = "127.0.0.1:3000".parse().unwrap();
  let server = Server::bind(&addr)
    .serve(|| service_fn(router))
    .map_err(|e| eprintln!("server error: {}", e));

  info!("server running on {:?}", addr);
  rt::spawn(server);
}

fn router(req: Request<Body>) -> ResponseFuture {
  let p = req.uri().to_owned();
  match (req.method(), p.path()) {
    (&Method::GET, "/") => home(),
    (&Method::GET, "/feeds") => index(),
    (&Method::GET, r) if r.starts_with("/static/") => show_asset(r),
    (&Method::GET, r) if r.starts_with("/feed/") => show_channel(r),
    (&Method::GET, r) if r.starts_with("/item/") => show_item(r),
    (&Method::GET, r) if r.starts_with("/items/") => show_items(r),
    (&Method::POST, "/add_feed") => add_feed(req.into_body()),
    (&Method::OPTIONS, _) => cors_headers(),
    _ => {
      info!("rejected {} {}", req.method(), p.path());
      Box::new(future::ok(
        Response::builder()
          .status(StatusCode::NOT_FOUND)
          .body(Body::empty())
          .unwrap(),
      ))
    }
  }
}

fn add_feed(body: Body) -> ResponseFuture {
  let reversed = body.concat2().map(move |chunk| {
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
  Box::new(reversed)
}

fn home() -> ResponseFuture {
  let mut f = File::open("vue/dist/index.html").unwrap();
  let mut buffer = String::new();
  f.read_to_string(&mut buffer).unwrap();
  Box::new(future::ok(
    Response::builder()
      .header("Access-Control-Allow-Origin", "*")
      .body(Body::from(buffer))
      .unwrap(),
  ))
}

fn index() -> ResponseFuture {
  let channels = get_channels();
  let mut body = Body::empty();
  let mut status = StatusCode::OK;
  match serde_json::to_string(&channels) {
    Ok(json) => {
      body = Body::from(json);
    }
    Err(_) => {
      status = StatusCode::NOT_FOUND;
    }
  };
  Box::new(future::ok(
    Response::builder()
      .status(status)
      .header("Access-Control-Allow-Origin", "*")
      .body(body)
      .unwrap(),
  ))
}

fn show_channel(req_path: &str) -> ResponseFuture {
  let re = Regex::new(r"/feed/(\d+)").unwrap();
  let ch_id = match re.captures(req_path) {
    Some(d) => d.get(1).unwrap().as_str(),
    None => {
      info!("no match: {}", req_path);
      return Box::new(future::ok(Response::new(Body::empty())));
    }
  };

  let content = match get_channel_with_items(ch_id.parse::<i32>().unwrap()) {
    Some(data) => match serde_json::to_string(&data) {
      Ok(json) => Response::new(Body::from(json)),
      Err(_) => Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap(),
    },
    None => Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(Body::empty())
      .unwrap(),
  };
  Box::new(future::ok(content))
}

fn show_item(req_path: &str) -> ResponseFuture {
  let re = Regex::new(r"/item/(\d+)").unwrap();
  let ch_id = match re.captures(req_path) {
    Some(d) => d.get(1).unwrap().as_str(),
    None => {
      info!("no match: {}", req_path);
      return Box::new(future::ok(Response::new(Body::empty())));
    }
  };

  let content = match get_item(ch_id.parse::<i32>().unwrap()) {
    Some(data) => match serde_json::to_string(&data) {
      Ok(json) => Response::new(Body::from(json)),
      Err(_) => Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap(),
    },
    None => Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(Body::empty())
      .unwrap(),
  };
  Box::new(future::ok(content))
}

fn show_items(req_path: &str) -> ResponseFuture {
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
  let content = match serde_json::to_string(&data) {
    Ok(json) => body = Body::from(json),
    Err(_) => status = StatusCode::NOT_FOUND,
  };
  Box::new(future::ok(
    Response::builder()
      .status(status)
      .header("Access-Control-Allow-Origin", "*")
      .body(body)
      .unwrap(),
  ))
}

fn show_asset(req_path: &str) -> ResponseFuture {
  let re = Regex::new(r"/static/(.+)").unwrap();
  let d = match re.captures(req_path) {
    Some(d) => d.get(1).unwrap().as_str(),
    None => {
      info!("no param match");
      return Box::new(future::ok(Response::new(Body::empty())));
    }
  };

  let f = path::Path::new("vue/dist/static").join(d);

  Box::new(
    tokio_fs::file::File::open(f)
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
            .body("Not found".into())
            .unwrap(),
        )
      }),
  )
}

fn cors_headers() -> ResponseFuture {
  let mut headers = HeaderMap::new();
  Box::new(future::ok(
    Response::builder()
      .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
      .header(ACCESS_CONTROL_EXPOSE_HEADERS, "Access-Control-*")
      .header(
        ACCESS_CONTROL_ALLOW_HEADERS,
        "Access-Control-*, Origin, X-Requested-With, Content-Type, Accept",
      )
      .header(
        ACCESS_CONTROL_ALLOW_METHODS,
        "GET, POST, PUT, DELETE, OPTIONS, HEAD",
      )
      .header(ALLOW, "GET, POST, PUT, DELETE, OPTIONS, HEAD")
      .body(Body::empty())
      .unwrap(),
  ))
}
