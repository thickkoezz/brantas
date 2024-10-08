use crate::config::CFG;
use anyhow::Error;
use sea_orm::{entity::prelude::DatabaseConnection, ConnectOptions, Database};
use std::time::Duration;
use tokio::sync::OnceCell;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init_db_conn() {
  DB.get_or_init(|| async {
    let mut opt = ConnectOptions::new(CFG.database.database_url.to_owned());
    opt
      .max_connections(1000)
      .min_connections(5)
      .connect_timeout(Duration::from_secs(8))
      .idle_timeout(Duration::from_secs(8))
      .sqlx_logging(false);

    Database::connect(opt)
      .await
      .expect("Failed to open database")
  })
  .await;
}

pub fn get_db() -> Result<&'static DatabaseConnection, Error> {
  DB.get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))
}
