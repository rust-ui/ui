use dioxus::prelude::*;
use uuid::Uuid;

use crate::domain::item::item_db::Item;

#[cfg(feature = "server")]
async fn pool() -> Result<sqlx::PgPool, ServerFnError> {
    use dioxus::server::axum::Extension;
    use dioxus_fullstack::FullstackContext;

    let Extension(pool) = FullstackContext::extract::<Extension<sqlx::PgPool>, _>().await?;
    Ok(pool)
}

#[server]
pub async fn get_items() -> Result<Vec<Item>, ServerFnError> {
    let pool = pool().await?;
    Item::get_all(&pool).await.map_err(ServerFnError::new)
}

#[server]
pub async fn get_item(id: Uuid) -> Result<Option<Item>, ServerFnError> {
    let pool = pool().await?;
    Item::get_by_id(&pool, id).await.map_err(ServerFnError::new)
}

#[server]
pub async fn create_item(title: String, description: Option<String>) -> Result<Item, ServerFnError> {
    let pool = pool().await?;
    Item::create(&pool, title, description).await.map_err(ServerFnError::new)
}

#[server]
pub async fn delete_item(id: Uuid) -> Result<Uuid, ServerFnError> {
    let pool = pool().await?;
    Item::delete(&pool, id).await.map_err(ServerFnError::new)
}
