use askama::Template;
use futures::{future, Future, Stream};
use hyper::rt;
use hyper::service::service_fn;
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use regex::Regex;
use std::collections::HashMap;
use std::{path, str};
use tokio_fs;
use tokio_io;
use url::form_urlencoded;

use db::{get_channel_with_items, get_channels, get_item};
use feed;
use template::{FeedChannelTemplate, FeedItemTemplate, IndexTemplate};

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
    (&Method::GET, "/") | (&Method::GET, "/feeds") => index(),
    (&Method::GET, r) if r.starts_with("/dist/") => show_asset(r),
    (&Method::GET, r) if r.starts_with("/feed/") => show_channel(r),
    (&Method::GET, r) if r.starts_with("/post/") => show_post(r),
    (&Method::POST, "/add_feed") => add_feed(req.into_body()),
    _ => Box::new(future::ok(
      Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap(),
    )),
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

fn index() -> ResponseFuture {
  let channels = get_channels();
  let feed = IndexTemplate::new(&channels);
  let res = match feed.render() {
    Ok(feed_content) => Response::new(Body::from(feed_content)),
    Err(_) => Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(Body::empty())
      .unwrap(),
  };
  Box::new(future::ok(res))
  // Box::new(future::ok(Response::new(Body::from("hello world"))))
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

  match get_channel_with_items(ch_id.parse::<i32>().unwrap()) {
    Some(data) => {
      let feed = FeedChannelTemplate::new(&data);
      let feed_html = feed.render().unwrap();
      Box::new(future::ok(
        Response::builder().body(Body::from(feed_html)).unwrap(),
      ))
    }
    None => {
      info!("not found!");
      Box::new(future::ok(
        Response::builder()
          .status(StatusCode::NOT_FOUND)
          .body("Not found".into())
          .unwrap(),
      ))
    }
  }
}

fn show_post(req_path: &str) -> ResponseFuture {
  let re = Regex::new(r"/post/(\d+)").unwrap();
  let ch_id = match re.captures(req_path) {
    Some(d) => d.get(1).unwrap().as_str(),
    None => {
      info!("no match: {}", req_path);
      return Box::new(future::ok(Response::new(Body::empty())));
    }
  };

  match get_item(ch_id.parse::<i32>().unwrap()) {
    Some(data) => {
      let feed = FeedItemTemplate::new(&data);
      let feed_html = feed.render().unwrap();
      Box::new(future::ok(
        Response::builder().body(Body::from(feed_html)).unwrap(),
      ))
    }
    None => {
      info!("not found!");
      Box::new(future::ok(
        Response::builder()
          .status(StatusCode::NOT_FOUND)
          .body("Not found".into())
          .unwrap(),
      ))
    }
  }
}

fn show_asset(req_path: &str) -> ResponseFuture {
  let re = Regex::new(r"/dist/(.+)").unwrap();
  let d = match re.captures(req_path) {
    Some(d) => d.get(1).unwrap().as_str(),
    None => {
      info!("no param match");
      return Box::new(future::ok(Response::new(Body::empty())));
    }
  };

  let f = path::Path::new("dist").join(d);

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
