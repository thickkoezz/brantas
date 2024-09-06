use sea_orm::EntityTrait;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Friend;
use super::DeletionMode;

pub async fn delete_chat_group(
  deletion_mode: DeletionMode,
  invitor_id: Uuid,
  invitee_id: Uuid
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  Friend::delete_by_id((invitor_id, invitee_id)).exec(db).await?;
  Ok(())
}
