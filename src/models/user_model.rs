use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub address: String,
    pub hash: String,
    pub users_brought: u32,
    pub created_at: String,
    pub last_user_brought_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPub {
    pub address: String,
    // pub hash: String,
    pub users_brought: u32,
    pub created_at: String,
    pub last_user_brought_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWalletAddress {
    pub address: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserHash {
    pub hash: String,
}
