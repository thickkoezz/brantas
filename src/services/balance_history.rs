use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::balance_history;
use crate::entities::prelude::BalanceHistory;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_balance_history(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
  ref_id: Uuid,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = BalanceHistory::delete_by_id((
        owner_id,
        created_at,
        ref_id
      ))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("balance_history"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let balance_history = BalanceHistory::find_by_id((
        owner_id,
        created_at,
        ref_id
      )).one(db).await?;
      if balance_history.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("balance_history"))).into());
      }

      let mut balance_history: balance_history::ActiveModel = balance_history.unwrap().into();
      balance_history.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      balance_history.update(db).await?;
      Ok(())
    }
  }
}
