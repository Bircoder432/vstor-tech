// src/main.rs
mod backend;
mod domain;
mod shared;

use vstor_tech::backend::config::Config;

#[tokio::main]
async fn main() {
    // Загружаем конфиг из .env
    let config = Config::from_env().expect("❌ Failed to load configuration");

    println!("🚀 Starting server on port {}", config.port);
    println!("📊 Database type: {:?}", config.database.db_type);

    if config.access_token.is_some() {
        println!("🔑 Access token: configured");
    } else {
        println!("⚠️  Access token: not configured (using 'default_token')");
    }

    // Создаем приложение
    let app = backend::api::create_app();

    // Запускаем сервер с портом из конфига
    let listener = tokio::net::TcpListener::bind(config.server_address())
        .await
        .expect("❌ Failed to bind to address");

    println!("✅ Server started on http://{}", config.server_address());
    axum::serve(listener, app).await.unwrap();
}
