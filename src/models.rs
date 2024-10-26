use serde::{Deserialize,Serialize};

#[derive(Hash, Debug, Deserialize, Serialize, Clone)]
pub struct PartialUser {
    pub username: String,
    pub password: String,
}
#[derive(Hash, Debug, Deserialize, Serialize, Clone)]
pub struct Response {
    pub status:String,
    pub message:String,
}
#[derive(Hash, Debug, Deserialize, Serialize, Clone)]
pub struct EmailUser {
    pub username: String,
    pub password: String,
    pub email:String
}