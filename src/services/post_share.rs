use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::PostShare;
use crate::entities::post_share;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_post_share(
  deletion_mode: DeletionMode,
  post_owner_id: Uuid,
  post_created_at: DateTimeWithTimeZone,
  target_id: Uuid,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = PostShare::delete_by_id((
        post_owner_id,
        post_created_at,
        target_id,
      ))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("user"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let post_share = PostShare::find_by_id((
        post_owner_id,
        post_created_at,
        target_id,
      ))
        .one(db)
        .await?;
      if post_share.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user"))).into());
      }

      let mut post_share: post_share::ActiveModel = post_share.unwrap().into();
      post_share.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      post_share.update(db).await?;
      Ok(())
    }
  }
}
