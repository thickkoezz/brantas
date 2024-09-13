use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct DirectChatReactionAddRequest {
  pub owner_id: Uuid,
  pub reacted_direct_chat_sender_id: Uuid,
  pub reacted_direct_chat_receiver_id: Uuid,
  pub reacted_direct_chat_created_at: DateTimeWithTimeZone,
  pub reaction_emoji: String,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct DirectChatReactionResponse {
  pub owner_id: Uuid,
  pub reacted_direct_chat_sender_id: Uuid,
  pub reacted_direct_chat_receiver_id: Uuid,
  pub reacted_direct_chat_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  pub reaction_emoji: String,
}

impl From<crate::entities::direct_chat_reaction::Model> for DirectChatReactionResponse {
  fn from(m: crate::entities::direct_chat_reaction::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      reacted_direct_chat_sender_id: m.reacted_direct_chat_sender_id,
      reacted_direct_chat_receiver_id: m.reacted_direct_chat_receiver_id,
      reacted_direct_chat_created_at: m.reacted_direct_chat_created_at,
      created_at: m.created_at,
      reaction_emoji: m.reaction_emoji,
    }
  }
}
