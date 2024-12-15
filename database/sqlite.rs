use sqlx::SqlitePool;

pub async fn initialize_database(pool: &SqlitePool) {
    sqlx::query("DROP TABLE IF EXISTS users")
        .execute(pool)
        .await.expect("");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )
        "#,
    )
        .execute(pool)
        .await.expect("");
}