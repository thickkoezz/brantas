use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::chat_group;
use crate::entities::prelude::ChatGroup;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_chat_group(
  deletion_mode: DeletionMode,
  creator_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = ChatGroup::delete_by_id((creator_id, created_at))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("chat_group"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let chat_group = ChatGroup::find_by_id((creator_id, created_at))
        .one(db).await?;
      if chat_group.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("chat_group"))).into());
      }

      let mut chat_group: chat_group::ActiveModel = chat_group.unwrap().into();
      chat_group.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      chat_group.update(db).await?;
      Ok(())
    }
  }
}
