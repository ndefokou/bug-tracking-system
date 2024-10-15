use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bug {
    pub _id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResult {
    pub _id: Option<ObjectId>,
    pub bug_id: ObjectId,
    pub result: String,
    pub timestamp: String,
}
