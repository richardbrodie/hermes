use super::types::{LoginParams, SettingsData};
use db::{create_user, get_user};
use models::{Claims, User};
use std::str;

// pub fn change_settings(settings: &SettingsData, claims: &Claims) -> Result<(), ()> {

//   match settings.name.as_ref() {
//     "add_user" => {
//       // let login = LoginParams {
//       //   username: settings.data.get("username").unwrap().to_owned(),
//       //   password: settings.data.get("password").unwrap().to_owned(),
//       // };
//       Ok(())
//     }
//     "change_password" => {
//       // let pw = settings.data.get("new_password").unwrap().to_owned();
//       // match change_password(&claims.name, &pw) {
//       //   Ok(_) => Ok(()),
//       //   Err(_) => Err(()),
//       // }
//       Ok(())
//     }
//     _ => Err(()),
//   }
// }

pub fn add_user(login: &LoginParams, claims: &Claims) -> Result<(), ()> {
  if claims.id != 1 {
    return Err(());
  };
  match get_user(&login.username) {
    None => {
      let pwh = User::hash_pw(&login.password);
      match create_user(&login.password, &pwh) {
        Ok(_) => Ok(()),
        Err(_e) => Err(()),
      }
    }
    Some(_) => Err(()),
  }
}

pub fn change_password(_un: &str, _pw: &str) -> Result<(), ()> {
  Ok(())
}
