use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct StarredChatResponseAddRequest {
  pub creator_id: Uuid,
  pub chat_sender_id: Option<Uuid>,
  pub direct_chat_receiver_id: Option<Uuid>,
  pub group_chat_group_creator_id: Option<Uuid>,
  pub group_chat_group_created_at: Option<DateTimeWithTimeZone>,
  pub chat_created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct StarredChatResponse {
  pub creator_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub chat_sender_id: Option<Uuid>,
  pub direct_chat_receiver_id: Option<Uuid>,
  pub group_chat_group_creator_id: Option<Uuid>,
  pub group_chat_group_created_at: Option<DateTimeWithTimeZone>,
  pub chat_created_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::starred_chat> for StarredChatResponse {
  fn from(m: crate::entities::starred_chat) -> StarredChatResponse {
    StarredChatResponse {
      creator_id: m.creator_id,
      created_at: m.created_at,
      chat_sender_id: m.chat_sender_id,
      direct_chat_receiver_id: m.direct_chat_receiver_id,
      group_chat_group_creator_id: m.group_chat_group_creator_id,
      group_chat_group_created_at: m.group_chat_group_created_at,
      chat_created_at: m.chat_created_at,
    }
  }
}
