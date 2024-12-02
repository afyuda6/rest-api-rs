use serde::Serialize;
use warp::reply::Json;
use warp::Rejection;

#[derive(Serialize)]
pub struct Response {
    pub status: String,
    pub code: u16,
}

pub async fn handle_read_users() -> Result<Json, Rejection> {
    let response = Response {
        status: "OK".to_string(),
        code: 200,
    };
    Ok(warp::reply::json(&response))
}

pub async fn handle_create_user() -> Result<Json, Rejection> {
    let response = Response {
        status: "Created".to_string(),
        code: 201,
    };
    Ok(warp::reply::json(&response))
}

pub async fn handle_update_user() -> Result<Json, Rejection> {
    let response = Response {
        status: "OK".to_string(),
        code: 200,
    };
    Ok(warp::reply::json(&response))
}

pub async fn handle_delete_user() -> Result<Json, Rejection> {
    let response = Response {
        status: "OK".to_string(),
        code: 200,
    };
    Ok(warp::reply::json(&response))
}
