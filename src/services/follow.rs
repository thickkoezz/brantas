use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Follow;
use crate::entities::follow;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_follow(
  deletion_mode: DeletionMode,
  follower_id: Uuid,
  target_id: Uuid,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Follow::delete_by_id((follower_id, target_id))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("follow"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let follow = Follow::find_by_id((follower_id, target_id))
        .one(db).await?;
      if follow.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("follow"))).into());
      }

      let mut follow: follow::ActiveModel = follow.unwrap().into();
      follow.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      follow.update(db).await?;
      Ok(())
    },
  }
}
