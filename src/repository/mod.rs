mod objects;
mod storage;
use log::info;
// use anyhow::Ok;
pub use objects::task_dbo;
pub use objects::user::*;

use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Postgres, PgConnection, Connection};
pub use storage::*;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let url = dotenvy::var("DATABASE_URL").expect("'DATABASE_URL' is not specified");
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(5)
        .max_lifetime(Some(std::time::Duration::from_secs(60 * 60)))
        .connect(url.as_str())
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(pool)
}

pub async fn processing_migration()-> Result<(), sqlx::Error>{
    let url = &dotenvy::var("DATABASE_URL").expect("'DATABASE_URL' is not specified");
    if !Postgres::database_exists(url).await?{
        info!("Database not exists.");
        Postgres::create_database(url).await?;
        info!("Database created.");
    }
    let mut connection = PgConnection::connect(url).await?;
    sqlx::migrate!("./migrations")
    .run(&mut connection)
    .await
    .expect("Error while running database migrations.");

    Ok(())
}