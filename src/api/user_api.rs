use crate::{
    models::user_model::{User, UserHash, UserPub, UserWalletAddress},
    repository::mongodb_repo::MongoRepo,
};

use chrono::prelude::Utc;
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data = "<user>")]
pub fn get_or_create_user(
    db: &State<MongoRepo>,
    user: Json<UserWalletAddress>,
) -> Result<Json<User>, Status> {
    let address = user.address.clone();
    if address.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_or_create_user(&address);
    if user_detail.is_ok() {
        let user = user_detail.unwrap();
        return Ok(Json(user));
    } else {
        return Err(Status::InternalServerError);
    }
}

#[post("/update", data = "<user>")]
pub fn increment_users_brought(
    db: &State<MongoRepo>,
    user: Json<UserHash>,
) -> Result<Json<UserPub>, Status> {
    let user_detail = db.get_user_by_hash(&user.hash.clone());
    if user_detail.is_err() {
        return Err(Status::InternalServerError);
    }
    let user_obj = user_detail.unwrap();
    let timestamp = Utc::now().to_string();

    let data = User {
        address: user_obj.address.clone(),
        hash: user_obj.hash.clone(),
        users_brought: user_obj.users_brought + 1,
        created_at: user_obj.created_at.clone(),
        last_user_brought_at: timestamp.clone(),
        owns_nft: user_obj.owns_nft,
    };

    let data2 = UserPub {
        address: user_obj.address.clone(),
        users_brought: user_obj.users_brought + 1,
        created_at: user_obj.created_at.clone(),
        last_user_brought_at: timestamp.clone(),
    };
    let update_result = db.update_user(&user_obj.hash, data.clone());
    if update_result.is_err() {
        return Err(Status::InternalServerError);
    }
    return Ok(Json(data2));
}

#[get("/leaderboard")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<UserPub>>, Status> {
    let users = db.get_all_users();

    match users {
        Ok(users) => {
            let mut ups = vec![];
            for u in users {
                ups.push(UserPub {
                    address: u.address,
                    users_brought: u.users_brought,
                    last_user_brought_at: u.last_user_brought_at,
                    created_at: u.created_at,
                })
            }
            return Ok(Json(ups));
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
