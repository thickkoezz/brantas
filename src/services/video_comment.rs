use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::video_comment::VideoCommentDTO;
use crate::entities::{prelude::VideoComment, video_comment};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_video_comment(req: VideoCommentDTO) -> AppResult<VideoCommentDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = video_comment::ActiveModel {
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    commented_video_owner_id: Set(req.commented_video_owner_id),
    commented_video_created_at: Set(req.commented_video_created_at),
    updated_at: Set(None),
    deleted_at: Set(None),
    content: Set(req.content),
    reaction_count: Set(0),
  };
  let video_comment = VideoComment::insert(model.clone()).exec(db).await?;
  Ok(VideoCommentDTO {
    owner_id: video_comment.last_insert_id.0,
    created_at: video_comment.last_insert_id.1,
    ..VideoCommentDTO::from(model)
  })
}

pub async fn delete_video_comment(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = VideoComment::delete_by_id((owner_id, created_at))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("comment"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let video_comment = VideoComment::find_by_id((owner_id, created_at))
        .one(db)
        .await?;
      if video_comment.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("comment"))).into());
      }

      let mut video_comment: video_comment::ActiveModel = video_comment.unwrap().into();
      video_comment.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      video_comment.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_video_comment(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<VideoCommentDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let video_comments = VideoComment::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = video_comments
        .into_iter()
        .map(|video_comment: video_comment::Model| VideoCommentDTO::from(video_comment))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let video_comments = VideoComment::find().all(db).await?;
      let res = video_comments
        .into_iter()
        .map(|video_comment: video_comment::Model| VideoCommentDTO::from(video_comment))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
