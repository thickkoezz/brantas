use crate::services::PaginatorOption;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::PostBookmark, post_bookmark};
use crate::dtos::post_bookmark::PostBookmarkResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait};
use uuid::Uuid;

pub async fn delete_post_bookmark(
  owner_id: Uuid,
  bookmarked_post_owner_id: Uuid,
  bookmarked_post_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let result = PostBookmark::delete_by_id((
    owner_id, bookmarked_post_owner_id, bookmarked_post_created_at,
  )).exec(db).await?;
  match result.rows_affected {
    0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("bookmark"))).into()),
    _ => Ok(()),
  }
}

pub async fn get_post_bookmark(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<PostBookmarkResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let post_bookmarks = PostBookmark::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = post_bookmarks.into_iter()
        .map(|post_bookmark: post_bookmark::Model| PostBookmarkResponse::from(post_bookmark))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let post_bookmarks = PostBookmark::find().all(db).await?;
      let res = post_bookmarks.into_iter()
        .map(|post_bookmark: post_bookmark::Model| PostBookmarkResponse::from(post_bookmark))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
