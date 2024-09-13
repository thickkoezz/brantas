use crate::services::PaginatorOption;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::TweetBookmark, tweet_bookmark};
use crate::dtos::tweet_bookmark::TweetBookmarkResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait};
use uuid::Uuid;

pub async fn delete_tweet_bookmark(
  owner_id: Uuid,
  bookmarked_tweet_owner_id: Uuid,
  bookmarked_tweet_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let result = TweetBookmark::delete_by_id((
    owner_id, bookmarked_tweet_owner_id, bookmarked_tweet_created_at,
  )).exec(db).await?;
  match result.rows_affected {
    0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("bookmark"))).into()),
    _ => Ok(()),
  }
}

pub async fn get_tweet_bookmark(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<TweetBookmarkResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let tweet_bookmarks = TweetBookmark::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = tweet_bookmarks.into_iter()
        .map(|tweet_bookmark: tweet_bookmark::Model| TweetBookmarkResponse::from(tweet_bookmark))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let tweet_bookmarks = TweetBookmark::find().all(db).await?;
      let res = tweet_bookmarks.into_iter()
        .map(|tweet_bookmark: tweet_bookmark::Model| TweetBookmarkResponse::from(tweet_bookmark))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
