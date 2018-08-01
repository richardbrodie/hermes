extern crate diesel;
extern crate dotenv;
extern crate feeds_lib;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use feeds_lib::db::establish_pool;
use feeds_lib::models::User;

fn main() {
  use feeds_lib::schema::users::dsl::*;
  dotenv().ok();

  let pool = establish_pool();
  let connection = pool.get().unwrap();

  let args: Vec<String> = env::args().collect();

  match &args.len() {
    3 => {
      let usr = &args[1];
      let pwd = &args[2];

      let pwh = User::hash_pw(&pwd);
      diesel::insert_into(users)
        .values((username.eq(usr), password_hash.eq(&pwh.as_bytes())))
        .load::<User>(&*connection)
        .expect("Error creating user");
    }
    _ => println!("accepts only two args"),
  }
}
