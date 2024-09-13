use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PostReactionAddRequest {
  pub owner_id: Uuid,
  pub reacted_post_owner_id: Uuid,
  pub reacted_post_created_at: DateTimeWithTimeZone,
  pub reaction_emoji: String,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PostReactionResponse {
  pub owner_id: Uuid,
  pub reacted_post_owner_id: Uuid,
  pub reacted_post_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  pub reaction_emoji: String,
}

impl From<crate::entities::post_reaction::Model> for PostReactionResponse {
  fn from(m: crate::entities::post_reaction::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      reacted_post_owner_id: m.reacted_post_owner_id,
      reacted_post_created_at: m.reacted_post_created_at,
      created_at: m.created_at,
      reaction_emoji: m.reaction_emoji,
    }
  }
}
