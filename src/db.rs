use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::postgres::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct Product {
    id: i32,
    is_vegan: String,
    name: String,
    brand: Option<String>,
    certification: Option<String>,
    project_check: String,
    ingredients: Option<String>,
    additional: Option<String>,
    barcode: Option<String>,
    status: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Tags {
    id: i32,
    tag: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ProductTags {
    product_id: i32,
    tag_id: i32,
}

pub async fn all_products(
    pool: &PgPool,
    since: Option<DateTime<Utc>>,
) -> Result<Vec<Product>, sqlx::Error> {
    let rows = sqlx::query_as::<_, Product>("SELECT id, is_vegan, name, brand, certification, project_check, ingredients, additional, barcode, status, updated_at FROM products WHERE updated_at > $1")
         .bind(since.unwrap_or_default())
         .fetch_all(pool)
         .await? ;
    Ok(rows)
}
