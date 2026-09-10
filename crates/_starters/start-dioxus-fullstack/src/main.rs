#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]

mod app;
mod components;
mod domain;
mod utils;

use app::App;

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        use dioxus::server::axum::Extension;
        use sqlx::postgres::PgPoolOptions;

        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .log_statements(log::LevelFilter::Off)
            .log_slow_statements(log::LevelFilter::Warn, std::time::Duration::from_millis(100))
            .connect(&database_url)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

        Ok(dioxus::server::router(App).layer(Extension(pool)))
    });
}
