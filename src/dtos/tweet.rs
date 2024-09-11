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

impl From<crate::entities::tweet> for TweetResponse {
  fn from(m: crate::entities::tweet) -> TweetResponse {
    TweetResponse {
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
