use futures::{Future, Sink, Stream};
use serde_json;
use warp::ws::{Message, WebSocket};

use super::types::{
  ChangePasswordParams, IncomingMessage, IncomingMessageType, LoginParams, SettingsData,
  SubscribeParams, UserWebsocketState,
};

use db::mark_subscribed_item_as_read;
use feed;
use models::{Claims, FeedMessage, OutgoingWebsocketMessage};

pub fn ws_created(
  ws: WebSocket,
  claims: Claims,
  users: UserWebsocketState,
) -> impl Future<Item = (), Error = ()> {
  let user_id = claims.id;
  debug!("WS: user connected: {} - {}", user_id, claims.name);
  let (tx, rx) = ws.split();
  users.insert(user_id, tx);
  let users2 = users.clone();

  rx.for_each(move |msg| {
    Ok(match ws_incoming_msg(&claims, msg, &users) {
      Some(msg) => ws_send_message(&user_id, msg, &users),
      None => (),
    })
  }).then(move |result| {
    ws_user_disconnected(&user_id, &users2);
    result
  }).map_err(move |e| {
    error!("WS: connect error uid={}: {}", &user_id, e);
  })
}

pub fn ws_incoming_msg(
  claims: &Claims,
  msg: Message,
  users: &UserWebsocketState,
) -> Option<Message> {
  let user_id = claims.id;
  match serde_json::from_str::<IncomingMessage>(msg.to_str().unwrap()) {
    Ok(message) => match message.msg_type {
      IncomingMessageType::MarkRead => {
        let data = message.data.parse::<i32>().unwrap();
        info!("WS: user {} read item {}", user_id, data);
        mark_subscribed_item_as_read(data);
      }
      IncomingMessageType::Subscribe => {
        let data = serde_json::from_str::<SubscribeParams>(&message.data).unwrap();
        info!("WS: user {} subscribed to {:?}", user_id, data);
        feed::subscribe_feed(data, user_id, users.to_owned());
      }
      IncomingMessageType::AddUser => {
        let data = serde_json::from_str::<LoginParams>(&message.data).unwrap();
        info!("WS: user {} added new user: {:?}", user_id, data);
        let m = OutgoingWebsocketMessage::action_result(message.msg_type, true);
        match super::handlers::add_user(&data, claims) {
          Ok(_) => (),
          Err(e) => (),
        }
      }
      IncomingMessageType::ChangePassword => {
        let data = serde_json::from_str::<ChangePasswordParams>(&message.data).unwrap();
        info!("WS: user {} changed password: {:?}", user_id, data);
      }
      IncomingMessageType::ChangeSettings => {
        let data = serde_json::from_str::<SettingsData>(&message.data).unwrap();
        info!("WS: user {} changed settings: {:?}", user_id, data);
      }
    },
    Err(_) => error!("WS: could not parse {:?} as a IncomingMessage", msg),
  };
  None
}

pub fn ws_user_disconnected(user_id: &i32, users: &UserWebsocketState) {
  debug!("WS: user {} disconnected", user_id);
  users.remove(user_id);
}

pub fn ws_send_message(user_id: &i32, message: Message, state: &UserWebsocketState) {
  match state.state.lock().unwrap().get_mut(user_id) {
    Some(tx) => {
      let _ = tx.start_send(message);
    }
    None => (),
  };
}
