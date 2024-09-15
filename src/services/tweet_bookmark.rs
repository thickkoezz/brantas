use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::tweet_bookmark::{TweetBookmarkAddRequest, TweetBookmarkResponse};
use crate::entities::{prelude::TweetBookmark, tweet_bookmark};
use crate::services::PaginatorOption;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_tweet_bookmark(req: TweetBookmarkAddRequest) -> AppResult<TweetBookmarkResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = tweet_bookmark::ActiveModel {
    owner_id: Set(req.owner_id),
    bookmarked_tweet_owner_id: Set(req.bookmarked_tweet_owner_id),
    bookmarked_tweet_created_at: Set(req.bookmarked_tweet_created_at),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
  };
  let tweet_bookmark = TweetBookmark::insert(model.clone()).exec(db).await?;
  Ok(TweetBookmarkResponse {
    owner_id: tweet_bookmark.last_insert_id.0,
    bookmarked_tweet_owner_id: tweet_bookmark.last_insert_id.1,
    bookmarked_tweet_created_at: tweet_bookmark.last_insert_id.2,
    ..TweetBookmarkResponse::from(model)
  })
}

pub async fn delete_tweet_bookmark(
  owner_id: Uuid,
  bookmarked_tweet_owner_id: Uuid,
  bookmarked_tweet_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let result = TweetBookmark::delete_by_id((
    owner_id,
    bookmarked_tweet_owner_id,
    bookmarked_tweet_created_at,
  ))
  .exec(db)
  .await?;
  match result.rows_affected {
    0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("bookmark"))).into()),
    _ => Ok(()),
  }
}

pub async fn get_tweet_bookmark(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<TweetBookmarkResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let tweet_bookmarks = TweetBookmark::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = tweet_bookmarks
        .into_iter()
        .map(|tweet_bookmark: tweet_bookmark::Model| TweetBookmarkResponse::from(tweet_bookmark))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let tweet_bookmarks = TweetBookmark::find().all(db).await?;
      let res = tweet_bookmarks
        .into_iter()
        .map(|tweet_bookmark: tweet_bookmark::Model| TweetBookmarkResponse::from(tweet_bookmark))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
