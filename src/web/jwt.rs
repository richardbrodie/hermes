use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Header, Validation};
use std::env;
use warp;
use warp::http::StatusCode;

use super::types::LoginParams;
use models::{Claims, User};

pub fn authenticate(params: LoginParams) -> Result<impl warp::Reply, warp::Rejection> {
  match User::check_user(&params.username, &params.password) {
    Some(user) => {
      let jwt = generate_jwt(&user).unwrap();
      let json_body = json!({ "token": jwt, });
      Ok(warp::reply::json(&json_body))
    }
    _ => Err(warp::reject::bad_request()),
  }
}

pub fn decode_jwt(token: String) -> Result<Claims, StatusCode> {
  let secret = env::var("JWT_SECRET").unwrap();
  let t = token;

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

pub fn generate_jwt(user: &User) -> Option<String> {
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
