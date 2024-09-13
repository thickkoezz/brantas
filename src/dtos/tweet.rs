use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct TweetAddRequest {
  pub owner_id: Uuid,
  pub tweet: String,
  pub is_published: bool,
  pub hashtag: Option<String>,
  pub replied_owner_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub retweeted_owner_id: Option<Uuid>,
  pub retweeted_created_at: Option<DateTimeWithTimeZone>,
  pub reaction_count: i32,
  pub reply_count: i32,
  pub retweet_count: i32,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct TweetUpdateRequest {
  pub owner_id: Uuid,
  pub tweet: String,
  pub is_published: bool,
  pub hashtag: Option<String>,
  pub replied_owner_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub retweeted_owner_id: Option<Uuid>,
  pub retweeted_created_at: Option<DateTimeWithTimeZone>,
  pub reaction_count: i32,
  pub reply_count: i32,
  pub retweet_count: i32,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct TweetResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub tweet: String,
  pub is_published: bool,
  pub hashtag: Option<String>,
  pub replied_owner_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub retweeted_owner_id: Option<Uuid>,
  pub retweeted_created_at: Option<DateTimeWithTimeZone>,
  pub reaction_count: i32,
  pub reply_count: i32,
  pub retweet_count: i32,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::tweet::Model> for TweetResponse {
  fn from(m: crate::entities::tweet::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      tweet: m.tweet,
      is_published: m.is_published,
      hashtag: m.hashtag,
      replied_owner_id: m.replied_owner_id,
      replied_created_at: m.replied_created_at,
      retweeted_owner_id: m.retweeted_owner_id,
      retweeted_created_at: m.retweeted_created_at,
      reaction_count: m.reaction_count,
      reply_count: m.reply_count,
      retweet_count: m.retweet_count,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<crate::entities::tweet::ActiveModel> for TweetResponse {
  fn from(m: crate::entities::tweet::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      tweet: m.tweet.unwrap(),
      is_published: m.is_published.unwrap(),
      hashtag: m.hashtag.unwrap(),
      replied_owner_id: m.replied_owner_id.unwrap(),
      replied_created_at: m.replied_created_at.unwrap(),
      retweeted_owner_id: m.retweeted_owner_id.unwrap(),
      retweeted_created_at: m.retweeted_created_at.unwrap(),
      reaction_count: m.reaction_count.unwrap(),
      reply_count: m.reply_count.unwrap(),
      retweet_count: m.retweet_count.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
