use sea_orm::EntityTrait;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::GroupChat;
use super::DeletionMode;

pub async fn delete_group_chat(
  deletion_mode: DeletionMode,
  sender_id: Uuid,
  group_creator_id: Uuid,
  group_created_at: DateTimeWithTimeZone,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB.get().ok_or(
    anyhow::anyhow!(t!("database_connection_failed"))
  )?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = GroupChat::delete_by_id((
        sender_id,
        group_creator_id,
        group_created_at,
        created_at
      )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("group_chat"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
