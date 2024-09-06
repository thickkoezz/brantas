use sea_orm::EntityTrait;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::ChatGroupMember;
use super::DeletionMode;

pub async fn delete_chat_group_member(
  deletion_mode: DeletionMode,
  group_creator_id: Uuid,
  group_created_at: DateTimeWithTimeZone,
  member_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB.get().ok_or(
    anyhow::anyhow!(t!("database_connection_failed"))
  )?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = ChatGroupMember::delete_by_id((
        group_creator_id,
        group_created_at,
        member_id,
        created_at
      )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("chat_group_member"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
