use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
}

#[cfg(feature = "server")]
mod db {
    use sqlx::PgPool;

    use super::*;

    impl Item {
        pub async fn get_all(pool: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
            sqlx::query_as!(Item, "SELECT id, title, description FROM items ORDER BY created_at DESC")
                .fetch_all(pool)
                .await
        }

        pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Item>, sqlx::Error> {
            sqlx::query_as!(Item, "SELECT id, title, description FROM items WHERE id = $1", id)
                .fetch_optional(pool)
                .await
        }

        pub async fn create(pool: &PgPool, title: String, description: Option<String>) -> Result<Item, sqlx::Error> {
            sqlx::query_as!(
                Item,
                "INSERT INTO items (title, description) VALUES ($1, $2) RETURNING id, title, description",
                title,
                description
            )
            .fetch_one(pool)
            .await
        }

        pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Uuid, sqlx::Error> {
            sqlx::query_scalar!("DELETE FROM items WHERE id = $1 RETURNING id", id).fetch_one(pool).await
        }
    }
}
