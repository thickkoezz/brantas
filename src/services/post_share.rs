use sea_orm::EntityTrait;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::PostShare;
use super::DeletionMode;

pub async fn delete_post_share(
  deletion_mode: DeletionMode,
  post_owner_id: Uuid,
  post_created_at: DateTimeWithTimeZone,
  target_id: Uuid,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(
        anyhow::anyhow!(t!("database_connection_failed"))
      )?;
      let result = PostShare::delete_by_id((
        post_owner_id,
        post_created_at,
        target_id,
      )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("person"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
