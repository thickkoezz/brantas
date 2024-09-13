use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct GroupChatReactionAddRequest {
  pub owner_id: Uuid,
  pub reacted_direct_chat_sender_id: Uuid,
  pub reacted_direct_chat_receiver_id: Uuid,
  pub reacted_direct_chat_created_at: DateTimeWithTimeZone,
  pub reaction_emoji: String,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct GroupChatReactionResponse {
  pub owner_id: Uuid,
  pub reacted_group_chat_sender_id: Uuid,
  pub reacted_group_chat_group_creator_id: Uuid,
  pub reacted_group_chat_group_created_at: DateTimeWithTimeZone,
  pub reacted_group_chat_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  pub reaction_emoji: String,
}

impl From<crate::entities::group_chat_reaction::Model> for GroupChatReactionResponse {
  fn from(m: crate::entities::group_chat_reaction::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      reacted_group_chat_sender_id: m.reacted_group_chat_sender_id,
      reacted_group_chat_group_creator_id: m.reacted_group_chat_group_creator_id,
      reacted_group_chat_group_created_at: m.reacted_group_chat_group_created_at,
      reacted_group_chat_created_at: m.reacted_group_chat_created_at,
      created_at: m.created_at,
      reaction_emoji: m.reaction_emoji,
    }
  }
}

impl From<crate::entities::group_chat_reaction::ActiveModel> for GroupChatReactionResponse {
  fn from(m: crate::entities::group_chat_reaction::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      reacted_group_chat_sender_id: m.reacted_group_chat_sender_id.unwrap(),
      reacted_group_chat_group_creator_id: m.reacted_group_chat_group_creator_id.unwrap(),
      reacted_group_chat_group_created_at: m.reacted_group_chat_group_created_at.unwrap(),
      reacted_group_chat_created_at: m.reacted_group_chat_created_at.unwrap(),
      created_at: m.created_at.unwrap(),
      reaction_emoji: m.reaction_emoji.unwrap(),
    }
  }
}
