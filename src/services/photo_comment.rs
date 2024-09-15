use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::photo_comment::{PhotoCommentAddRequest, PhotoCommentResponse};
use crate::entities::{photo_comment, prelude::PhotoComment};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_photo_comment(req: PhotoCommentAddRequest) -> AppResult<PhotoCommentResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = photo_comment::ActiveModel {
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    commented_photo_owner_id: Set(req.commented_photo_owner_id),
    commented_photo_created_at: Set(req.commented_photo_created_at),
    updated_at: Set(None),
    deleted_at: Set(None),
    content: Set(req.content),
    reaction_count: Set(0),
  };
  let photo_comment = PhotoComment::insert(model.clone()).exec(db).await?;
  Ok(PhotoCommentResponse {
    owner_id: photo_comment.last_insert_id.0,
    created_at: photo_comment.last_insert_id.1,
    ..PhotoCommentResponse::from(model)
  })
}

pub async fn delete_photo_comment(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = PhotoComment::delete_by_id((owner_id, created_at))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("comment"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let photo_comment = PhotoComment::find_by_id((owner_id, created_at))
        .one(db)
        .await?;
      if photo_comment.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("comment"))).into());
      }

      let mut photo_comment: photo_comment::ActiveModel = photo_comment.unwrap().into();
      photo_comment.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      photo_comment.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_photo_comment(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<PhotoCommentResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let photo_comments = PhotoComment::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = photo_comments
        .into_iter()
        .map(|photo_comment: photo_comment::Model| PhotoCommentResponse::from(photo_comment))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let photo_comments = PhotoComment::find().all(db).await?;
      let res = photo_comments
        .into_iter()
        .map(|photo_comment: photo_comment::Model| PhotoCommentResponse::from(photo_comment))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
