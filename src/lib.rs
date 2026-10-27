mod db;

use axum::{Json, Router, extract::Query, extract::State, http::StatusCode, routing::get};
use chrono::{DateTime, Utc};
use db::{Product, all_products};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct ProductQuery {
    since: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
struct SyncedDB {
    synced_at: DateTime<Utc>,
    products: Vec<Product>,
}

#[tokio::main]
pub async fn run() {
    dotenvy::dotenv().ok(); // .ok() перевірка існування файлу

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL має бути встановлена");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("не вдалось підключити бд");
    sqlx::migrate!("./src/migrations")
        .run(&pool)
        .await
        .expect("не вдалось виконати міграції");

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/products", get(products_handler))
        .with_state(pool);

    // Get the port number from the environment, default to 3000
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string()) // Get the port as a string or default to "3000"
        .parse() // Parse the port string into a u16
        .expect("Failed to parse PORT");

    // Create a socket address (IPv6 binding)
    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("не вдалося запустити tcp listener");

    axum::serve(listener, app)
        .await
        .expect("не вдалося запустити сервер");
}

async fn health_handler() -> &'static str {
    "Ok"
}

async fn products_handler(
    State(pool): State<PgPool>,
    product_query: Query<ProductQuery>,
) -> Result<Json<SyncedDB>, StatusCode> {
    match all_products(&pool, product_query.since).await {
        Ok(products) => {
            let synced_at = match products.iter().map(|n| n.updated_at).max() {
                Some(time) => time,
                None => product_query.since.unwrap_or_default(),
            };
            Ok(Json(SyncedDB {
                synced_at,
                products,
            }))
        }
        Err(e) => {
            eprintln!("Не вдалося отримати перелік продуктів: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
