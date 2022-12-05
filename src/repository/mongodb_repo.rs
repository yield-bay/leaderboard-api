use std::env;

extern crate dotenv;

use chrono::prelude::Utc;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, extjson::de::Error},
    options::FindOneAndUpdateOptions,
    results::UpdateResult,
    sync::{Client, Collection},
};
use sha2::{Digest, Sha256};

use crate::models::user_model::User;
pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let dbname = match env::var("DBNAME") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(&dbname);
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub fn get_or_create_user(&self, address: &String) -> Result<User, Error> {
        let mut hasher = Sha256::new();
        hasher.update(address.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        println!("hash: {:?}", hash.clone());
        let timestamp = Utc::now().to_string();

        let filter = doc! {"address": address};
        let u = doc! {
            "$set" : {
                "address": address.clone(),
                "hash": hash.clone(),
                "users_brought": 0,
                "created_at": timestamp.clone(),
                "last_user_brought_at": "",
            }
        };
        let options = FindOneAndUpdateOptions::builder()
            .upsert(Some(true))
            .build();

        let udo = self.col.find_one(filter.clone(), None).unwrap();

        if udo.is_some() {
            return Ok(udo.unwrap());
        } else {
            println!("udo not ok");
            let _user_detail = self
                .col
                .find_one_and_update(filter, u, Some(options))
                .ok()
                .expect("Error getting user's detail");
            return Ok(User {
                address: address.to_string(),
                hash: hash,
                users_brought: 0,
                created_at: timestamp,
                last_user_brought_at: "".to_string(),
            });
        }
    }

    pub fn get_user_by_hash(&self, hash: &String) -> Result<User, Error> {
        let filter = doc! {"hash": hash};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn update_user(&self, hash: &String, new_user: User) -> Result<UpdateResult, Error> {
        let filter = doc! {"hash": hash};
        let new_doc = doc! {
            "$set":
                {
                    "address": new_user.address,
                    "hash": new_user.hash,
                    "users_brought": new_user.users_brought,
                    "created_at": new_user.created_at,
                    "last_user_brought_at": new_user.last_user_brought_at,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();

        Ok(users)
    }
}
