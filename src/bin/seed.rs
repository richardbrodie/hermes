extern crate argon2rs;
extern crate diesel;
extern crate feeds_lib;

use argon2rs::defaults::{KIB, LANES, PASSES};
use argon2rs::verifier::Encoded;
use argon2rs::{Argon2, Variant};
use diesel::prelude::*;

use feeds_lib::db::establish_connection;
use feeds_lib::schema::users::dsl::*;

fn main() {
  let userpass = "admin";
  let password_salt = "mmm, salt";

  let connection = establish_connection();
  diesel::delete(users)
    .execute(&connection)
    .expect("Error deleting users");

  let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2i).unwrap();
  let enc0 = Encoded::new(a2, userpass.as_bytes(), password_salt.as_bytes(), b"", b"");
  let pw_hash = String::from_utf8(enc0.to_u8()).unwrap();

  let _ = diesel::insert_into(users)
    .values((username.eq(userpass), password_hash.eq(pw_hash)))
    .execute(&connection);
}
