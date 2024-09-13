use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Post, post};
use crate::dtos::post::PostResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_post(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = Post::delete_by_id((owner_id, created_at)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("post"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let post = Post::find_by_id((owner_id, created_at)).one(db).await?;
      if post.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("post"))).into());
      }

      let mut post: post::ActiveModel = post.unwrap().into();
      post.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      post.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_post(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<PostResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let posts = Post::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = posts.into_iter().map(|post: post::Model| PostResponse::from(post))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let posts = Post::find().all(db).await?;
      let res = posts.into_iter().map(|post: post::Model| PostResponse::from(post))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
