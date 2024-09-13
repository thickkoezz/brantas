use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Tweet, tweet};
use crate::dtos::tweet::TweetResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_tweet(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = Tweet::delete_by_id((owner_id, created_at)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("tweet"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let tweet = Tweet::find_by_id((owner_id, created_at,)).one(db).await?;
      if tweet.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("tweet"))).into());
      }

      let mut tweet: tweet::ActiveModel = tweet.unwrap().into();
      tweet.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      tweet.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_tweet(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<TweetResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let tweets = Tweet::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = tweets.into_iter()
        .map(|tweet: tweet::Model| TweetResponse::from(tweet)).collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let tweets = Tweet::find().all(db).await?;
      let res = tweets.into_iter()
        .map(|tweet: tweet::Model| TweetResponse::from(tweet)).collect::<Vec<_>>();
      Ok(res)
    }
  }
}
