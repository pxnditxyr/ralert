use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};
use std::str::FromStr;

use crate::infrastructure::config::environments::ENVIRONMENTS;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url = &ENVIRONMENTS.database_url;

    let connection_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
}
