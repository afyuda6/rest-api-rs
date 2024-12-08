use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::FromRow;
use sqlx::{Row, SqlitePool};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::{
    reply::{with_status, Json},
    Rejection,
};

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

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub id: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    pub id: Option<String>,
}

pub async fn handle_read_users(
    pool: Arc<SqlitePool>,
) -> Result<warp::reply::WithStatus<Json>, Rejection> {
    let users: Vec<User> = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&*pool)
        .await
        .map_err(|_e| warp::reject::not_found())?;

    let response = ResponseWithData {
        status: "OK".to_string(),
        code: 200,
        data: users,
    };

    Ok(with_status(warp::reply::json(&response), StatusCode::OK))
}

pub async fn handle_create_user(
    pool: Arc<SqlitePool>,
    create_user_request: CreateUserRequest,
) -> Result<warp::reply::WithStatus<Json>, Rejection> {
    let name = create_user_request.name;

    if name.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
        let response = ResponseWithErrors {
            status: "Bad Request".to_string(),
            code: 400,
            errors: "Missing 'name' parameter".to_string(),
        };
        return Ok(with_status(
            warp::reply::json(&response),
            StatusCode::BAD_REQUEST,
        ));
    }

    match sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(&name)
        .execute(&*pool)
        .await
    {
        Ok(_) => {
            let response = Response {
                status: "Created".to_string(),
                code: 201,
            };
            Ok(with_status(
                warp::reply::json(&response),
                StatusCode::CREATED,
            ))
        }
        Err(_err) => {
            let response = Response {
                status: "Bad Request".to_string(),
                code: 400,
            };
            Ok(with_status(
                warp::reply::json(&response),
                StatusCode::BAD_REQUEST,
            ))
        }
    }
}

pub async fn handle_update_user(
    pool: Arc<SqlitePool>,
    update_user_request: UpdateUserRequest,
) -> Result<warp::reply::WithStatus<Json>, Rejection> {
    let id = update_user_request.id;
    let name = update_user_request.name;

    if id.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true)
        || name.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true)
    {
        let response = ResponseWithErrors {
            status: "Bad Request".to_string(),
            code: 400,
            errors: "Missing 'id' or 'name' parameter".to_string(),
        };
        return Ok(with_status(
            warp::reply::json(&response),
            StatusCode::BAD_REQUEST,
        ));
    }

    match sqlx::query("UPDATE users SET name = ? WHERE id = ?")
        .bind(&name)
        .bind(&id)
        .execute(&*pool)
        .await
    {
        Ok(_) => {
            let response = Response {
                status: "OK".to_string(),
                code: 200,
            };
            Ok(with_status(warp::reply::json(&response), StatusCode::OK))
        }
        Err(_err) => {
            let response = Response {
                status: "Bad Request".to_string(),
                code: 400,
            };
            Ok(with_status(
                warp::reply::json(&response),
                StatusCode::BAD_REQUEST,
            ))
        }
    }
}

pub async fn handle_delete_user(
    pool: Arc<SqlitePool>,
    delete_user_request: DeleteUserRequest,
) -> Result<warp::reply::WithStatus<Json>, Rejection> {
    let id = delete_user_request.id;

    if id.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
        let response = ResponseWithErrors {
            status: "Bad Request".to_string(),
            code: 400,
            errors: "Missing 'id' parameter".to_string(),
        };
        return Ok(with_status(
            warp::reply::json(&response),
            StatusCode::BAD_REQUEST,
        ));
    }

    match sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
    {
        Ok(_) => {
            let response = Response {
                status: "OK".to_string(),
                code: 200,
            };
            Ok(with_status(warp::reply::json(&response), StatusCode::OK))
        }
        Err(_err) => {
            let response = Response {
                status: "Bad Request".to_string(),
                code: 400,
            };
            Ok(with_status(
                warp::reply::json(&response),
                StatusCode::BAD_REQUEST,
            ))
        }
    }
}
