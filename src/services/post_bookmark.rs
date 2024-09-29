use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::post_bookmark::PostBookmarkDTO;
use crate::entities::{post_bookmark, prelude::PostBookmark};
use crate::services::PaginatorOption;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_post_bookmark(req: PostBookmarkDTO) -> AppResult<PostBookmarkDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = post_bookmark::ActiveModel {
    owner_id: Set(req.owner_id),
    bookmarked_post_owner_id: Set(req.bookmarked_post_owner_id),
    bookmarked_post_created_at: Set(req.bookmarked_post_created_at),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
  };
  let post_bookmark = PostBookmark::insert(model.clone()).exec(db).await?;
  Ok(PostBookmarkDTO {
    owner_id: post_bookmark.last_insert_id.0,
    bookmarked_post_owner_id: post_bookmark.last_insert_id.1,
    bookmarked_post_created_at: post_bookmark.last_insert_id.2,
    ..PostBookmarkDTO::from(model)
  })
}

pub async fn delete_post_bookmark(
  owner_id: Uuid,
  bookmarked_post_owner_id: Uuid,
  bookmarked_post_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let result = PostBookmark::delete_by_id((
    owner_id,
    bookmarked_post_owner_id,
    bookmarked_post_created_at,
  ))
  .exec(db)
  .await?;
  match result.rows_affected {
    0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("bookmark"))).into()),
    _ => Ok(()),
  }
}

pub async fn get_post_bookmark(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<PostBookmarkDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let post_bookmarks = PostBookmark::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = post_bookmarks
        .into_iter()
        .map(|post_bookmark: post_bookmark::Model| PostBookmarkDTO::from(post_bookmark))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let post_bookmarks = PostBookmark::find().all(db).await?;
      let res = post_bookmarks
        .into_iter()
        .map(|post_bookmark: post_bookmark::Model| PostBookmarkDTO::from(post_bookmark))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
