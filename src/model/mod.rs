use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::{NaiveTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub body: String,
    pub to: ObjectId,
    pub from: ObjectId,
    pub time: String,
}
