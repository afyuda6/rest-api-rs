use sqlx::SqlitePool;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::reply::{with_status, Json};
use warp::{Filter, Rejection};

mod database;
use database::sqlite::initialize_database;

mod handlers;
use crate::handlers::user::Response;
use handlers::user::{
    handle_create_user, handle_delete_user, handle_read_users, handle_update_user,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to SQLite database");

    initialize_database(&pool).await;

    let pool = Arc::new(pool);

    let get_users_route = warp::path("users")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and_then(handle_read_users);

    let post_user_route = warp::path("users")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_pool(pool.clone()))
        .and(warp::body::form())
        .and_then(handle_create_user);

    let put_user_route = warp::path("users")
        .and(warp::path::end())
        .and(warp::put())
        .and(with_pool(pool.clone()))
        .and(warp::body::form())
        .and_then(handle_update_user);

    let delete_user_route = warp::path("users")
        .and(warp::path::end())
        .and(warp::delete())
        .and(with_pool(pool.clone()))
        .and(warp::body::form())
        .and_then(handle_delete_user);

    let users = get_users_route
        .or(post_user_route)
        .or(put_user_route)
        .or(delete_user_route);

    let not_found = warp::any().and_then(handle_not_found);

    let method_not_allowed = warp::path("users")
        .and(warp::path::end())
        .and(warp::method())
        .and_then(handle_method_not_allowed);

    let routes = users.or(method_not_allowed).or(not_found);

    warp::serve(routes).run(([127, 0, 0, 1], 6007)).await;
}

fn with_pool(
    pool: Arc<SqlitePool>,
) -> impl Filter<Extract = (Arc<SqlitePool>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn handle_not_found() -> Result<warp::reply::WithStatus<Json>, Rejection> {
    let response = Response {
        status: "Not Found".to_string(),
        code: 404,
    };
    Ok(with_status(
        warp::reply::json(&response),
        StatusCode::NOT_FOUND,
    ))
}

async fn handle_method_not_allowed(
    _method: warp::http::Method,
) -> Result<warp::reply::WithStatus<Json>, Rejection> {
    let response = Response {
        status: "Method Not Allowed".to_string(),
        code: 405,
    };
    Ok(with_status(
        warp::reply::json(&response),
        StatusCode::METHOD_NOT_ALLOWED,
    ))
}
