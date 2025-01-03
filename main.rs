use std::net::{TcpListener};
use std::sync::{Arc, Mutex};
use sqlx::SqlitePool;

mod database;
use database::sqlite::initialize_database;

mod handlers;
use handlers::user::{handle_request};

fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("");
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("");
    runtime.block_on(async {
        let pool = SqlitePool::connect(&database_url)
            .await
            .expect("");
        let pool = Arc::new(Mutex::new(pool));
        initialize_database(&pool.lock().unwrap()).await;
        let port = std::env::var("PORT").unwrap_or_else(|_| "6007".to_string());
        let address = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(address).expect("");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let pool = pool.clone();
                    std::thread::spawn(move || handle_request(stream, pool));
                }
                _ => {}
            }
        }
    });
}