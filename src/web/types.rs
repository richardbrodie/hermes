use futures::stream::SplitSink;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use warp::ws::WebSocket;
use warp::{self, Rejection};

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
}

#[derive(Deserialize, Debug)]
pub struct SettingsData {
  pub data: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct LoginParams {
  pub username: String,
  pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ChangePasswordParams {
  pub username: String,
  pub old_pass: String,
  pub new_pass: String,
}

#[derive(Deserialize, Debug)]
pub struct SubscribeParams {
  pub feed_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum IncomingMessageType {
  MarkRead,
  Subscribe,
  AddUser,
  ChangePassword,
  ChangeSettings,
}

#[derive(Deserialize, Debug)]
pub struct IncomingMessage {
  pub msg_type: IncomingMessageType,
  pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct AccessToken {
  pub access_token: String,
}

pub struct AssetFile(pub String);
impl FromStr for AssetFile {
  type Err = Rejection;
  fn from_str(s: &str) -> Result<AssetFile, Rejection> {
    // let re = Regex::new(r"((?:src|favicon)\.\w+\.(?:css|js|png))").unwrap();
    let re = Regex::new(r"((?:main|favicon2).(?:css|js|png))").unwrap();
    match re.captures(&s) {
      Some(m) => Ok(AssetFile(m.get(1).unwrap().as_str().to_owned())),
      None => Err(warp::reject::not_found()),
    }
  }
}
