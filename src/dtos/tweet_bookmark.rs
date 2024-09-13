use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct TweetBookmarkAddRequest {
  pub owner_id: Uuid,
  pub bookmarked_tweet_owner_id: Uuid,
  pub bookmarked_tweet_created_at: DateTimeWithTimeZone,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct TweetBookmarkResponse {
  pub owner_id: Uuid,
  pub bookmarked_tweet_owner_id: Uuid,
  pub bookmarked_tweet_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
}

impl From<crate::entities::tweet_bookmark::Model> for TweetBookmarkResponse {
  fn from(m: crate::entities::tweet_bookmark::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      bookmarked_tweet_owner_id: m.bookmarked_tweet_owner_id,
      bookmarked_tweet_created_at: m.bookmarked_tweet_created_at,
      created_at: m.created_at,
    }
  }
}

impl From<crate::entities::tweet_bookmark::ActiveModel> for TweetBookmarkResponse {
  fn from(m: crate::entities::tweet_bookmark::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      bookmarked_tweet_owner_id: m.bookmarked_tweet_owner_id.unwrap(),
      bookmarked_tweet_created_at: m.bookmarked_tweet_created_at.unwrap(),
      created_at: m.created_at.unwrap(),
    }
  }
}
