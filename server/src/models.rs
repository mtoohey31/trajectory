use super::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub double_hashed_passwd: String,
    pub hash_salt: String,
    pub data: Option<String>,
    pub vault_salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreds {
    pub username: String,
    pub hashed_passwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataUpdate {
    pub username: String,
    pub hashed_passwd: String,
    pub new_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub data: Option<String>,
    pub vault_salt: String,
}
