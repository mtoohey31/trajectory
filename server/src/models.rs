use super::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub double_hashed_passwd: String,
    pub hash_salt: Vec<u8>,
    pub data: Option<Vec<u8>>,
    pub vault_salt: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreds {
    pub username: String,
    pub hashed_passwd: String,
}
