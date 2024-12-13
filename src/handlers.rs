use warp::Filter;
use sqlx::MySqlPool;
use bcrypt::{hash, DEFAULT_COST};  // bcrypt library for hashing passwords
use crate::models::{User, NewUser, Message, NewMessage};
use warp::{http::StatusCode, reply};


pub fn create_routes(pool: MySqlPool) -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    let pool_filter = warp::any().map(move || pool.clone());

    let add_user = warp::path("add_user")
        .and(warp::post())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(add_user_handler);

    let add_message = warp::path("add_message")
        .and(warp::post())
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(add_message_handler);

    add_user.or(add_message).boxed()
}

async fn add_user_handler(new_user: NewUser, pool: MySqlPool) -> Result<impl warp::Reply, warp::Rejection> {
    // Hash the password using bcrypt before saving
    match hash(new_user.password, DEFAULT_COST) {
        Ok(password_hash) => {
            match sqlx::query!(
                "INSERT INTO users (username, password_hash) VALUES (?, ?)", 
                new_user.username, 
                password_hash
            )
            .execute(&pool)
            .await
            {
                Ok(_) => Ok(reply::with_status(
                    reply::json(&serde_json::json!({"message": "User added"})),
                    StatusCode::CREATED,
                )),
                Err(_) => Ok(reply::with_status(
                    reply::json(&serde_json::json!({"error": "Failed to add user"})),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            }
        }
        Err(_) => Ok(warp::reply::with_status(
            reply::json(&serde_json::json!({"error": "Failed to hash password"})), 
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

async fn add_message_handler(new_message: NewMessage, pool: MySqlPool) -> Result<impl warp::Reply, warp::Rejection> {
    match sqlx::query!(
        "INSERT INTO messages (sender_id, receiver_id, message) VALUES (?, ?, ?)", 
        new_message.sender_id, 
        new_message.receiver_id,
        new_message.message
    )
    .execute(&pool)
    .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            reply::json(&serde_json::json!({"message": "Message added"})), 
            StatusCode::CREATED,
        )),
        Err(_) => Ok(warp::reply::with_status(
            reply::json(&serde_json::json!({"error": "Failed to add message"})), 
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}