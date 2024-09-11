use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct GroupChatAddRequest {
  pub sender_id: Uuid,
  pub group_creator_id: Uuid,
  pub group_created_at: DateTimeWithTimeZone,
  pub content: String,
  pub replied_sender_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_sender_id: Option<Uuid>,
  pub forwarded_group_creator_id: Option<Uuid>,
  pub forwarded_group_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_receiver_id: Option<Uuid>,
  pub is_pinned: bool,
  pub pin_expired_at: Option<DateTimeWithTimeZone>,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct GroupChatUpdateRequest {
  pub sender_id: Uuid,
  pub group_creator_id: Uuid,
  pub group_created_at: DateTimeWithTimeZone,
  pub content: String,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub replied_sender_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_sender_id: Option<Uuid>,
  pub forwarded_group_creator_id: Option<Uuid>,
  pub forwarded_group_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_receiver_id: Option<Uuid>,
  pub is_pinned: bool,
  pub pin_expired_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct GroupChatResponse {
  pub sender_id: Uuid,
  pub group_creator_id: Uuid,
  pub group_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  pub content: String,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub replied_sender_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_sender_id: Option<Uuid>,
  pub forwarded_group_creator_id: Option<Uuid>,
  pub forwarded_group_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_receiver_id: Option<Uuid>,
  pub is_pinned: bool,
  pub pin_expired_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::group_chat> for GroupChatResponse {
  fn from(m: crate::entities::group_chat) -> GroupChatResponse {
    GroupChatResponse {
      sender_id: m.sender_id,
      group_creator_id: m.group_creator_id,
      group_created_at: m.group_created_at,
      created_at: m.created_at,
      content: m.content,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      replied_sender_id: m.replied_sender_id,
      replied_created_at: m.replied_created_at,
      forwarded_sender_id: m.forwarded_sender_id,
      forwarded_group_creator_id: m.forwarded_group_creator_id,
      forwarded_group_created_at: m.forwarded_group_created_at,
      forwarded_created_at: m.forwarded_created_at,
      forwarded_receiver_id: m.forwarded_receiver_id,
      is_pinned: m.is_pinned,
      pin_expired_at: m.pin_expired_at,
    }
  }
}
