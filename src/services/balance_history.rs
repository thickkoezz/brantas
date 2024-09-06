use sea_orm::EntityTrait;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::BalanceHistory;
use super::DeletionMode;

pub async fn delete_balance_history(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
  ref_id: Uuid
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      BalanceHistory::delete_by_id((owner_id,created_at, ref_id)).exec(db).await?;
      Ok(())
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
