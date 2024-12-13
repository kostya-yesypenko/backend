use warp::Filter;
use db::init_db;
use std::sync::{Arc, Mutex};

mod db;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = init_db().await.expect("Failed to initialize the database");

    // Create routes and start the server
    let api = handlers::create_routes(pool);

    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
