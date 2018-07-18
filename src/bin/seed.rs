extern crate diesel;
extern crate feeds_lib;
extern crate sodiumoxide;

use diesel::prelude::*;
use sodiumoxide::crypto::pwhash;

use feeds_lib::db::establish_connection;
use feeds_lib::schema::users::dsl::*;

fn main() {
  let userpass = "admin";

  let connection = establish_connection();
  diesel::delete(users)
    .execute(&connection)
    .expect("Error deleting users");

  let pwh = pwhash::pwhash(
    userpass.as_bytes(),
    pwhash::OPSLIMIT_INTERACTIVE,
    pwhash::MEMLIMIT_INTERACTIVE,
  ).unwrap();

  diesel::insert_into(users)
    .values((username.eq(userpass), password_hash.eq(&pwh[..])))
    .execute(&connection)
    .expect("Error inserting to db");
}
