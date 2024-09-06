use sea_orm::EntityTrait;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::DirectChat;
use super::DeletionMode;

pub async fn delete_direct_chat(
  deletion_mode: DeletionMode,
  sender_id: Uuid,
  receiver_id: Uuid,
  created_at: DateTimeWithTimeZone
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      DirectChat::delete_by_id((sender_id, receiver_id, created_at)).exec(db).await?;
      Ok(())
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
