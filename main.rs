use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use sqlx::SqlitePool;

mod database;
use database::sqlite::initialize_database;

mod handlers;
use handlers::user::{
    handle_create_user, handle_delete_user, handle_read_users, handle_update_user,
};

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

        let listener = TcpListener::bind("127.0.0.1:6007").expect("");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let pool = pool.clone();
                    std::thread::spawn(move || handle_client(stream, pool));
                }
                _ => {}
            }
        }
    });
}

fn handle_client(mut stream: TcpStream, pool: Arc<Mutex<SqlitePool>>) {
    let mut buffer = [0; 1024];
    if let Ok(size) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..size]);
        let response = route_request(&request, pool);

        stream
            .write_all(response.as_bytes())
            .expect("");
    }
}

fn route_request(request: &str, pool: Arc<Mutex<SqlitePool>>) -> String {
    let lines: Vec<&str> = request.lines().collect();
    if lines.is_empty() {
        return http_response(400, "Bad Request", "{\"error\":\"Invalid Request\"}");
    }

    let parts: Vec<&str> = lines[0].split_whitespace().collect();
    if parts.len() < 2 {
        return http_response(400, "Bad Request", "{\"error\":\"Invalid Request\"}");
    }

    let method = parts[0];
    let path = parts[1];

    match (method, path) {
        ("GET", "/users") | ("GET", "/users/") => handle_read_users(pool),
        ("POST", "/users") | ("POST", "/users/") => handle_create_user(&lines, pool),
        ("PUT", "/users") | ("PUT", "/users/") => handle_update_user(&lines, pool),
        ("DELETE", "/users") | ("DELETE", "/users/") => handle_delete_user(&lines, pool),

        (_, "/users") | (_, "/users/") => {
            http_response(405, "Method Not Allowed", "{\"status\": \"Method Not Allowed\", \"code\": 405}")
        }

        _ => http_response(404, "Not Found", "{\"status\": \"Not Found\", \"code\": 404}"),
    }
}

fn http_response(status_code: u16, status: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        status,
        body.len(),
        body
    )
}
