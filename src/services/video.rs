use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Video, video};
use crate::dtos::video::VideoResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{
  ActiveModelTrait,
  EntityTrait,
  PaginatorTrait,
  Set,
};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_video(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
  video: String,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Video::delete_by_id((owner_id, created_at, video)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("video"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let video = Video::find_by_id((
        owner_id, created_at, video,
      )).one(db).await?;
      if video.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("video"))).into());
      }

      let mut video: video::ActiveModel = video.unwrap().into();
      video.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      video.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_video(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<VideoResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let videos = Video::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = videos.into_iter()
        .map(|video: video::Model| VideoResponse::from(video)).collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let videos = Video::find().all(db).await?;
      let res = videos.into_iter()
        .map(|video: video::Model| VideoResponse::from(video)).collect::<Vec<_>>();
      Ok(res)
    }
  }
}
