use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::tweet::TweetDTO;
use crate::entities::{prelude::Tweet, tweet};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_tweet(req: TweetDTO) -> AppResult<TweetDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = tweet::ActiveModel {
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    tweet: Set(req.tweet),
    is_published: Set(req.is_published),
    hashtag: Set(req.hashtag),
    replied_owner_id: Set(req.replied_owner_id),
    replied_created_at: Set(req.replied_created_at),
    retweeted_owner_id: Set(req.retweeted_owner_id),
    retweeted_created_at: Set(req.retweeted_created_at),
    reaction_count: Set(0),
    reply_count: Set(0),
    retweet_count: Set(0),
    updated_at: Set(None),
    deleted_at: Set(None),
  };
  let tweet = Tweet::insert(model.clone()).exec(db).await?;
  Ok(TweetDTO {
    owner_id: tweet.last_insert_id.0,
    created_at: tweet.last_insert_id.1,
    ..TweetDTO::from(model)
  })
}

pub async fn delete_tweet(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Tweet::delete_by_id((owner_id, created_at)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("tweet"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let tweet = Tweet::find_by_id((owner_id, created_at)).one(db).await?;
      if tweet.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("tweet"))).into());
      }

      let mut tweet: tweet::ActiveModel = tweet.unwrap().into();
      tweet.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      tweet.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_tweet(paginator_option: Option<PaginatorOption>) -> AppResult<Vec<TweetDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let tweets = Tweet::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = tweets
        .into_iter()
        .map(|tweet: tweet::Model| TweetDTO::from(tweet))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let tweets = Tweet::find().all(db).await?;
      let res = tweets
        .into_iter()
        .map(|tweet: tweet::Model| TweetDTO::from(tweet))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
