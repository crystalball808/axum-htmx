use anyhow::Result;
use sqlx::{FromRow, Row, SqlitePool};

#[derive(FromRow, Debug)]
pub struct User {
    id: i32,
    email: String,
    name: String,
    password: String,
}

#[derive(FromRow, Debug)]
pub struct Session {
    user_id: i32,
}

#[derive(FromRow, Debug)]
pub struct Post {
    pub id: i32,
    pub body: String,
    pub author_id: i32,
}

pub async fn init() -> Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let connection_pool = SqlitePool::connect(&database_url).await?;

    sqlx::migrate!().run(&connection_pool).await?;

    Ok(connection_pool)
}

pub async fn check_session_id(connection_pool: &SqlitePool, session_id: i32) -> Result<bool> {
    let result = sqlx::query_as::<_, Session>("SELECT user_id FROM sessions WHERE id=$1")
        .bind(session_id)
        .fetch_optional(connection_pool)
        .await?;

    return Ok(result.is_some());
}

#[derive(FromRow, Debug)]
pub struct UserId {
    pub id: i32,
}
pub async fn get_user_id_from_login(
    connection_pool: &SqlitePool,
    email: &str,
    password: &str,
) -> Result<Option<i32>> {
    let result = sqlx::query_as::<_, UserId>("SELECT id FROM users WHERE email=$1 AND password=$2")
        .bind(email)
        .bind(password)
        .fetch_optional(connection_pool)
        .await?;

    Ok(match result {
        Some(user) => Some(user.id),
        None => None,
    })
}

pub async fn check_email_exists(connection_pool: &SqlitePool, email: &str) -> Result<bool> {
    let result = sqlx::query_as::<_, User>("SELECT id FROM users WHERE email=$1")
        .bind(email)
        .fetch_optional(connection_pool)
        .await?;

    Ok(result.is_some())
}

pub async fn create_user(
    connection_pool: &SqlitePool,
    email: &str,
    name: &str,
    password: &str,
) -> Result<()> {
    sqlx::query("INSERT INTO users (email, name, password) VALUES ($1, $2, $3)")
        .bind(email)
        .bind(name)
        .bind(password)
        .execute(connection_pool)
        .await?;
    Ok(())
}

pub async fn create_session(connection_pool: &SqlitePool, user_id: i32) -> Result<i32> {
    Ok(
        sqlx::query("INSERT INTO sessions (user_id) VALUES ($1) RETURNING id")
            .bind(user_id)
            .fetch_one(connection_pool)
            .await?
            .get(0),
    )
}

pub async fn get_posts(connection_pool: &SqlitePool) -> Result<Vec<Post>> {
    Ok(
        sqlx::query_as::<_, Post>("SELECT id, body, author_id FROM posts")
            .fetch_all(connection_pool)
            .await?,
    )
}
