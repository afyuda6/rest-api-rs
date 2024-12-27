use serde::Serialize;
use sqlx::{query, query_as, SqlitePool};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize)]
pub struct Response {
    pub status: String,
    pub code: u16,
}

#[derive(Serialize)]
pub struct ResponseWithData {
    pub status: String,
    pub code: u16,
    pub data: Vec<User>,
}

#[derive(Serialize)]
pub struct ResponseWithErrors {
    pub status: String,
    pub code: u16,
    pub errors: String,
}

pub fn handle_read_users(pool: Arc<Mutex<SqlitePool>>) -> String {
    let pool = pool.lock().unwrap();
    let users: Vec<User> = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(query_as!(User, "SELECT id, name FROM users").fetch_all(&*pool))
        .unwrap_or_default();
    let response = ResponseWithData {
        status: "OK".to_string(),
        code: 200,
        data: users,
    };
    let body = serde_json::to_string(&response).unwrap_or_default();
    http_response(200, "OK", &body)
}

pub fn handle_create_user(request: &[&str], pool: Arc<Mutex<SqlitePool>>) -> String {
    let body = extract_body(request);
    if body.is_empty() {
        return http_response(400, "Bad Request", "{\"status\": \"Bad Request\", \"code\": 400, \"errors\": \"Missing 'name' parameter\"}");
    }
    let form_data: Vec<(String, String)> = form_urlencoded::parse(body.as_bytes())
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
    let name = form_data.iter().find(|(key, _)| key == "name").map(|(_, value)| value.clone());
    if let Some(name) = name {
        if name.trim().is_empty() {
            return http_response(400, "Bad Request", "{\"status\": \"Bad Request\", \"code\": 400, \"errors\": \"Missing 'name' parameter\"}");
        }
        let pool = pool.lock().unwrap();
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            query("INSERT INTO users (name) VALUES (?)")
                .bind(&name)
                .execute(&*pool),
        );
        match result {
            Ok(_) => http_response(201, "Created", "{\"status\": \"Created\", \"code\": 201}"),
            _ => http_response(0, "", "")
        }
    } else {
        http_response(400, "Bad Request", "{\"status\": \"Bad Request\", \"code\": 400, \"errors\":\"Missing 'name' parameter\"}")
    }
}

pub fn handle_update_user(request: &[&str], pool: Arc<Mutex<SqlitePool>>) -> String {
    let body = extract_body(request);
    if body.is_empty() {
        return http_response(400, "Bad Request", "{\"status\": \"Bad Request\", \"code\": 400, \"errors\": \"Missing 'id' or 'name' parameter\"}");
    }
    let form_data: Vec<(String, String)> = form_urlencoded::parse(body.as_bytes())
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
    let name = form_data.iter().find(|(key, _)| key == "name").map(|(_, value)| value.clone());
    let id = form_data.iter().find(|(key, _)| key == "id").map(|(_, value)| value.clone());
    if let (Some(name), Some(id)) = (name, id) {
        if name.trim().is_empty() || id.trim().is_empty() {
            return http_response(400, "Bad Request", "{\"status\": \"Bad Request\", \"code\": 400, \"errors\": \"Missing 'id' or 'name' parameter\"}");
        }
        let pool = pool.lock().unwrap();
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            query("UPDATE users SET name = ? WHERE id = ?")
                .bind(&name)
                .bind(&id)
                .execute(&*pool),
        );
        match result {
            Ok(_) => http_response(200, "OK", "{\"status\": \"OK\", \"code\": 200}"),
            _ => http_response(0, "", "")
        }
    } else {
        http_response(400, "Bad Request", "{\"status\": \"Bad Request\", \"code\": 400, \"errors\":\"Missing 'id' or 'name' parameter\"}")
    }
}

pub fn handle_delete_user(request: &[&str], pool: Arc<Mutex<SqlitePool>>) -> String {
    let body = extract_body(request);
    if body.is_empty() {
        return http_response(400, "Bad Request", "{\"status\": \"Bad Request\", \"code\": 400, \"errors\": \"Missing 'id' parameter\"}");
    }
    let form_data: Vec<(String, String)> = form_urlencoded::parse(body.as_bytes())
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
    let id = form_data.iter().find(|(key, _)| key == "id").map(|(_, value)| value.clone());
    if let Some(id) = id {
        if id.trim().is_empty() {
            return http_response(400, "Bad Request", "{\"status\": \"Bad Request\", \"code\": 400, \"errors\": \"Missing 'id' parameter\"}");
        }
        let pool = pool.lock().unwrap();
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            query("DELETE FROM users WHERE id = ?")
                .bind(&id)
                .execute(&*pool),
        );
        match result {
            Ok(_) => http_response(200, "OK", "{\"status\": \"OK\", \"code\": 200}"),
            _ => http_response(0, "", "")
        }
    } else {
        http_response(400, "Bad Request", "{\"status\": \"Bad Request\", \"code\": 400, \"errors\":\"Missing 'id' parameter\"}")
    }
}

fn extract_body(request: &[&str]) -> String {
    request.iter().rev().find(|&&line| line.is_empty()).map(|_| request.last().unwrap_or(&"").to_string()).unwrap_or_default()
}

fn http_response(status_code: u16, status: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {} {}\r\n\
        Access-Control-Allow-Origin: *\r\n\
        Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n\
        Access-Control-Allow-Headers: Content-Type\r\n\
        Content-Type: application/json\r\n\
        Content-Length: {}\r\n\r\n\
        {}",
        status_code,
        status,
        body.len(),
        body
    )
}

pub(crate) fn handle_request(mut stream: TcpStream, pool: Arc<Mutex<SqlitePool>>) {
    let mut buffer = [0; 1024];
    if let Ok(size) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..size]);
        let lines: Vec<&str> = request.lines().collect();
        let parts: Vec<&str> = lines[0].split_whitespace().collect();
        let method = parts[0];
        let path = parts[1];
        let response = match (method, path) {
            ("GET", "/users") | ("GET", "/users/") => handle_read_users(pool),
            ("POST", "/users") | ("POST", "/users/") => handle_create_user(&lines, pool),
            ("PUT", "/users") | ("PUT", "/users/") => handle_update_user(&lines, pool),
            ("DELETE", "/users") | ("DELETE", "/users/") => handle_delete_user(&lines, pool),
            ("OPTIONS", "/users") | ("OPTIONS", "/users/") => {
                format!(
                    "HTTP/1.1 200 OK\r\n\
                     Access-Control-Allow-Origin: *\r\n\
                     Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n\
                     Access-Control-Allow-Headers: Content-Type\r\n\
                     Content-Type: application/json\r\n\r\n"
                )
            }
            (_, "/users") | (_, "/users/") => {
                http_response(405, "Method Not Allowed", "{\"status\": \"Method Not Allowed\", \"code\": 405}")
            }
            _ => http_response(404, "Not Found", "{\"status\": \"Not Found\", \"code\": 404}"),
        };
        stream
            .write_all(response.as_bytes())
            .expect("");
    }
}