use futures::{future, Future};
use hyper::{Body, Error, Method, Request, Response, StatusCode};
use regex::Regex;
use std::fmt;

pub type RequestFuture = fn(Request<Body>) -> ResponseFuture;
pub type ResponseFuture = Box<Future<Item = Response<Body>, Error = Error> + Send>;

#[derive(Debug, Clone)]
pub struct Route {
  verb: Method,
  route: Regex,
  with: RequestFuture,
}
impl Route {
  pub fn new(verb: Method, route: &str, with: RequestFuture) -> Route {
    let mut r = route.to_owned();
    if !r.starts_with("^") {
      r = format!("^{}", r);
    }
    if !r.ends_with("$") {
      r = format!("{}$", r);
    }

    let re = Regex::new(&r).unwrap();
    Route {
      verb: verb,
      route: re,
      with: with,
    }
  }
  pub fn matches(&self, path: &str) -> bool {
    self.route.is_match(path)
  }
}

#[derive(Debug, Clone)]
pub struct Router {
  routes: Vec<Route>,
}
impl Router {
  pub fn build() -> Router {
    let routes = Vec::new();
    Router { routes: routes }
  }

  pub fn route(&mut self, verb: Method, path: &str, with: RequestFuture) -> &mut Self {
    let route = Route::new(verb, path, with);
    self.routes.push(route);
    self
  }

  pub fn parse(&self, req: Request<Body>) -> ResponseFuture {
    let uri = req.uri().to_owned();
    let path = uri.path();

    for r in &self.routes {
      if r.matches(path) && &r.verb == req.method() {
        return (r.with)(req);
      }
    }
    Router::throw_code(StatusCode::NOT_FOUND)
  }

  pub fn throw_code(code: StatusCode) -> ResponseFuture {
    Router::response(Body::empty(), code)
  }

  pub fn response(body: Body, status: StatusCode) -> ResponseFuture {
    Box::new(future::ok(
      Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .status(status)
        .body(body)
        .unwrap(),
    ))
  }
}
