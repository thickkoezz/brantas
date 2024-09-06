use sea_orm::EntityTrait;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Follow;
use super::DeletionMode;

pub async fn delete_follow(
  deletion_mode: DeletionMode,
  follower_id: Uuid,
  target_id: Uuid
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      Follow::delete_by_id((follower_id, target_id)).exec(db).await?;
      Ok(())
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
