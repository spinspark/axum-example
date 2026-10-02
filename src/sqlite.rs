use anyhow::Context;
use sqlx::SqlitePool;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use std::str::FromStr;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations/sqlite");

pub async fn establish_pool(path: &str) -> anyhow::Result<SqlitePool> {
    let opts = SqliteConnectOptions::from_str(path)
        .with_context(|| format!("Invalid database path {path}"))?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);
    let pool = SqlitePool::connect_with(opts)
        .await
        .with_context(|| format!("Failed to open database at {path}"))?;

    MIGRATOR
        .run(&pool)
        .await
        .context("Failed to run migrations on database")?;

    Ok(pool)
}
