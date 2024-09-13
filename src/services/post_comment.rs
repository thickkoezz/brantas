use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::PostComment, post_comment};
use crate::dtos::post_comment::PostCommentResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_post_comment(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = PostComment::delete_by_id((
        owner_id, created_at
      )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("comment"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let post_comment = PostComment::find_by_id((
        owner_id, created_at,
      )).one(db).await?;
      if post_comment.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("comment"))).into());
      }

      let mut post_comment: post_comment::ActiveModel = post_comment.unwrap().into();
      post_comment.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      post_comment.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_post_comment(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<PostCommentResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let post_comments = PostComment::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = post_comments.into_iter()
        .map(|post_comment: post_comment::Model| PostCommentResponse::from(post_comment))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let post_comments = PostComment::find().all(db).await?;
      let res = post_comments.into_iter()
        .map(|post_comment: post_comment::Model| PostCommentResponse::from(post_comment))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
