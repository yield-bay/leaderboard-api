mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{get_all_users, get_or_create_user, increment_users_brought};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![get_or_create_user])
        .mount("/", routes![increment_users_brought])
        .mount("/", routes![get_all_users])
}
