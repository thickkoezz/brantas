use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::video::VideoDTO;
use crate::entities::{prelude::Video, video};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_video(req: VideoDTO) -> AppResult<VideoDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = video::ActiveModel {
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    video: Set(req.video),
    size: Set(req.size),
    updated_at: Set(None),
    deleted_at: Set(None),
    title: Set(req.title),
    caption: Set(req.caption),
    code: Set(req.code),
    slug: Set(req.slug),
    is_private: Set(req.is_private),
  };
  let video = Video::insert(model.clone()).exec(db).await?;
  Ok(VideoDTO {
    owner_id: video.last_insert_id.0,
    created_at: video.last_insert_id.1,
    ..VideoDTO::from(model)
  })
}

pub async fn delete_video(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Video::delete_by_id((owner_id, created_at)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("video"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let video = Video::find_by_id((owner_id, created_at)).one(db).await?;
      if video.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("video"))).into());
      }

      let mut video: video::ActiveModel = video.unwrap().into();
      video.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      video.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_video(paginator_option: Option<PaginatorOption>) -> AppResult<Vec<VideoDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let videos = Video::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = videos
        .into_iter()
        .map(|video: video::Model| VideoDTO::from(video))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let videos = Video::find().all(db).await?;
      let res = videos
        .into_iter()
        .map(|video: video::Model| VideoDTO::from(video))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
