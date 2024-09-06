use sea_orm::EntityTrait;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::FriendGroup;
use super::DeletionMode;

pub async fn delete_friend_group(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  name: String
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      FriendGroup::delete_by_id((owner_id, name)).exec(db).await?;
      Ok(())
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
