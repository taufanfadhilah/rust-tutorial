mod helpers;
mod user;

use axum::{routing::get, Router};
use sqlx::mysql::MySqlPoolOptions;
use tokio::net::TcpListener;
use user::{user_create, user_get, user_get_by_id, user_update, user_delete};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok().expect("Failed to load .env file");

    let server_address = std::env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let app = Router::new()
        .route("/users", get(user_get).post(user_create))
        .route("/users/{id}", get(user_get_by_id).put(user_update).delete(user_delete))
        .with_state(pool);

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Failed to bind to server address");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve application");
}
