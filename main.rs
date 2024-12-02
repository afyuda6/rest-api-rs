use sqlx::SqlitePool;
use warp::{Filter, Rejection};
use warp::reply::Json;

mod handlers;
use handlers::user::{handle_create_user, handle_read_users, handle_update_user, handle_delete_user};
use crate::handlers::user::Response;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let _pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let get_users_route = warp::path("users")
        .and(warp::get())
        .and_then(handle_read_users);

    let post_user_route = warp::path("users")
        .and(warp::post())
        .and_then(handle_create_user);

    let put_user_route = warp::path("users")
        .and(warp::put())
        .and_then(handle_update_user);

    let delete_user_route = warp::path("users")
        .and(warp::delete())
        .and_then(handle_delete_user);

    let users = get_users_route
        .or(post_user_route)
        .or(put_user_route)
        .or(delete_user_route);

    let not_found = warp::any().and_then(handle_not_found);

    let method_not_allowed = warp::path("users")
        .and(warp::method())
        .and_then(handle_method_not_allowed);

    let routes = users.or(method_not_allowed).or(not_found);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 6007))
        .await;
}

async fn handle_not_found() -> Result<Json, Rejection> {
    let response = Response {
        status: "Not Found".to_string(),
        code: 404,
    };
    Ok(warp::reply::json(&response))
}

async fn handle_method_not_allowed(_method: warp::http::Method) -> Result<Json, Rejection> {
    let response = Response {
        status: "Method Not Allowed".to_string(),
        code: 405,
    };
    Ok(warp::reply::json(&response))
}
