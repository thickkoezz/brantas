use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct TweetReactionAddRequest {
  pub owner_id: Uuid,
  pub reacted_tweet_owner_id: Uuid,
  pub reacted_tweet_created_at: DateTimeWithTimeZone,
  pub reaction_emoji: String,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct TweetReactionResponse {
  pub owner_id: Uuid,
  pub reacted_tweet_owner_id: Uuid,
  pub reacted_tweet_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  pub reaction_emoji: String,
}

impl From<crate::entities::tweet_reaction::Model> for TweetReactionResponse {
  fn from(m: crate::entities::tweet_reaction::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      reacted_tweet_owner_id: m.reacted_tweet_owner_id,
      reacted_tweet_created_at: m.reacted_tweet_created_at,
      created_at: m.created_at,
      reaction_emoji: m.reaction_emoji,
    }
  }
}

impl From<crate::entities::tweet_reaction::ActiveModel> for TweetReactionResponse {
  fn from(m: crate::entities::tweet_reaction::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      reacted_tweet_owner_id: m.reacted_tweet_owner_id.unwrap(),
      reacted_tweet_created_at: m.reacted_tweet_created_at.unwrap(),
      created_at: m.created_at.unwrap(),
      reaction_emoji: m.reaction_emoji.unwrap(),
    }
  }
}
