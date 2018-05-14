use actix;
use actix_web::http::{header, Method, StatusCode};
use actix_web::middleware::session::{self, CookieSessionBackend, RequestSession, SessionStorage};
use actix_web::{
  error, fs, middleware, pred, server, App, Error, HttpRequest, HttpResponse, Query, Result,
};
use std::collections::HashMap;

use askama::Template;
// use db::get_channels;
use template::FeedChannelTemplate;

fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
  Ok(
    HttpResponse::Ok()
      .content_type("text/html")
      // .body(fetch_channels()),
      .body("hello world"),
  )
}

pub fn start_web() {
  let sys = actix::System::new("basic-example");
  let addr = server::new(
        || App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            // cookie session middleware
            .middleware(SessionStorage::new(
                CookieSessionBackend::signed(&[0; 32]).secure(false)
            ))
            // static files
            .handler("/dist", fs::StaticFiles::new("dist"))
            // redirect
            .resource("/", |r| r.method(Method::GET).with(index))
            // default
            .default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(p404);

                // all requests that are not `GET`
                r.route().filter(pred::Not(pred::Get())).f(
                    |req| HttpResponse::MethodNotAllowed());
            }))

        .bind("127.0.0.1:8080").expect("Can not bind to 127.0.0.1:8080")
        .shutdown_timeout(0)    // <- Set shutdown timeout to 0 seconds (default 60s)
        .start();

  println!("Starting http server: 127.0.0.1:8080");
  let _ = sys.run();
}

// fn fetch_channels() -> String {
//   let channels = get_channels();
//   let feed = FeedChannelTemplate::new(&channels);
//   feed.render().unwrap()
// }

/// 404 handler
fn p404(req: HttpRequest) -> Result<fs::NamedFile> {
  Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}
