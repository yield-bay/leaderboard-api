mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use api::user_api::{get_all_users, get_or_create_user, increment_users_brought};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["https://list.yieldbay.io"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![get_or_create_user])
        .mount("/", routes![increment_users_brought])
        .mount("/", routes![get_all_users])
        .attach(cors)
}
