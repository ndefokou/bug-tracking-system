use axum::{Json, extract::Path};
use crate::db::models::{Bug};
use mongodb::Database;

pub async fn create_bug(
    Json(payload): Json<Bug>,
    db: Database
) -> Json<Bug> {
    // Logic to insert bug into MongoDB
}

pub async fn list_bugs(db: Database) -> Json<Vec<Bug>> {
    // Logic to fetch all bugs from MongoDB
}
