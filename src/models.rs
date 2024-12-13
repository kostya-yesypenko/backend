use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Message {
    pub id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: String,
}