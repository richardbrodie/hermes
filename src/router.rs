use futures::future;
use hyper::{Body, Method, Request, Response, StatusCode};
use regex::Regex;

use web::{
  AuthenticationHandler, ProtectedRequestFuture, RequestSignature, ResponseFuture,
  UnprotectedRequestFuture,
};

#[derive(Clone)]
pub struct Route {
  verb: Method,
  route: Regex,
  with: RequestSignature,
}
impl Route {
  pub fn new(verb: Method, route: &str, with: RequestSignature) -> Route {
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

#[derive(Clone)]
pub struct Router {
  routes: Vec<Route>,
  auth_handler: Option<AuthenticationHandler>,
}
impl Router {
  pub fn build() -> Router {
    let routes = Vec::new();
    Router {
      routes: routes,
      auth_handler: None,
    }
  }

  pub fn auth_handler(&mut self, handler: AuthenticationHandler) -> &mut Self {
    self.auth_handler = Some(handler);
    self
  }

  pub fn open_route(
    &mut self,
    verb: Method,
    path: &str,
    with: UnprotectedRequestFuture,
  ) -> &mut Self {
    let route = Route::new(verb, path, RequestSignature::Open(with));
    self.routes.push(route);
    self
  }

  pub fn closed_route(
    &mut self,
    verb: Method,
    path: &str,
    with: ProtectedRequestFuture,
  ) -> &mut Self {
    let route = Route::new(verb, path, RequestSignature::Closed(with));
    self.routes.push(route);
    self
  }

  pub fn parse(&self, req: Request<Body>) -> ResponseFuture {
    let uri = req.uri().to_owned();
    let path = uri.path();

    for r in &self.routes {
      if r.matches(path) && &r.verb == req.method() {
        return match r.with {
          RequestSignature::Open(fn1) => (fn1)(req),
          RequestSignature::Closed(fn2) => {
            let username = match (&self.auth_handler.unwrap())(&req) {
              Some(user) => user,
              None => return Router::throw_code(StatusCode::UNAUTHORIZED),
            };
            (fn2)(req, &username)
          }
        };
      }
    }
    info!("not found");
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
