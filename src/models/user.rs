use bcrypt::bcrypt;
use serde::{Deserialize, Serialize};
use validator::Validate;
use wither::bson::doc;
use wither::{bson::oid::ObjectId, Model};

#[derive(Debug, Model, Serialize, Deserialize, Validate)]
#[model(index(keys=r#"doc!{"email": 1}"#, options=r#"doc!{"unique": true}"#r))]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        let cost = 10;
        let hashed_password = bcrypt::hash(password, cost).unwrap();
        Self {
            id: None,
            username,
            email,
            password: hashed_password,
        }
    }
}
