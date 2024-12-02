use std::sync::Arc;
use serde::Serialize;
use sqlx::{Row, SqlitePool};
use sqlx::FromRow;
use sqlx::sqlite::SqliteRow;
use warp::reply::Json;
use warp::Rejection;

impl FromRow<'_, SqliteRow> for User {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
        })
    }
}

#[derive(Serialize)]
pub struct User {
    pub name: String,
    pub id: i64
}

#[derive(Serialize)]
pub struct Response {
    pub status: String,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<User>>
}

pub async fn handle_read_users(pool: Arc<SqlitePool>) -> Result<Json, Rejection> {
    let users: Vec<User> = vec![];

    let response = Response {
        status: "OK".to_string(),
        code: 200,
        data: if users.is_empty() { None } else { Some(users) },
    };

    Ok(warp::reply::json(&response))
}

pub async fn handle_create_user() -> Result<Json, Rejection> {
    let response = Response {
        status: "Created".to_string(),
        code: 201,
        data: None,
    };
    Ok(warp::reply::json(&response))
}

pub async fn handle_update_user() -> Result<Json, Rejection> {
    let response = Response {
        status: "OK".to_string(),
        code: 200,
        data: None,
    };
    Ok(warp::reply::json(&response))
}

pub async fn handle_delete_user() -> Result<Json, Rejection> {
    let response = Response {
        status: "OK".to_string(),
        code: 200,
        data: None,
    };
    Ok(warp::reply::json(&response))
}
