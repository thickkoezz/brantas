use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::video;
use crate::entities::prelude::Video;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_photo(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
  video: String,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Video::delete_by_id((owner_id, created_at, video))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("video"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let video = Video::find_by_id((
        owner_id,
        created_at,
        video,
      )).one(db).await?;
      if video.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("video"))).into());
      }

      let mut video: video::ActiveModel = video.unwrap().into();
      video.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      video.update(db).await?;
      Ok(())
    },
  }
}
