use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct DirectChatAddRequest {
  pub sender_id: Uuid,
  pub receiver_id: Uuid,
  pub content: String,
  pub replied_sender_id: Option<Uuid>,
  pub replied_receiver_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_sender_id: Option<Uuid>,
  pub forwarded_receiver_id: Option<Uuid>,
  pub forwarded_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_group_creator_id: Option<Uuid>,
  pub forwarded_group_created_at: Option<DateTimeWithTimeZone>,
  pub is_pinned: bool,
  pub pin_expired_at: Option<DateTimeWithTimeZone>,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct DirectChatUpdateRequest {
  pub sender_id: Uuid,
  pub receiver_id: Uuid,
  pub content: String,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub replied_sender_id: Option<Uuid>,
  pub replied_receiver_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_sender_id: Option<Uuid>,
  pub forwarded_receiver_id: Option<Uuid>,
  pub forwarded_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_group_creator_id: Option<Uuid>,
  pub forwarded_group_created_at: Option<DateTimeWithTimeZone>,
  pub is_pinned: bool,
  pub pin_expired_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct DirectChatResponse {
  pub sender_id: Uuid,
  pub receiver_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub content: String,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub replied_sender_id: Option<Uuid>,
  pub replied_receiver_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_sender_id: Option<Uuid>,
  pub forwarded_receiver_id: Option<Uuid>,
  pub forwarded_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_group_creator_id: Option<Uuid>,
  pub forwarded_group_created_at: Option<DateTimeWithTimeZone>,
  pub is_pinned: bool,
  pub pin_expired_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::direct_chat::Model> for DirectChatResponse {
  fn from(m: crate::entities::direct_chat::Model) -> Self {
    Self {
      sender_id: m.sender_id,
      receiver_id: m.receiver_id,
      created_at: m.created_at,
      content: m.content,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      replied_sender_id: m.replied_sender_id,
      replied_receiver_id: m.replied_receiver_id,
      replied_created_at: m.replied_created_at,
      forwarded_sender_id: m.forwarded_sender_id,
      forwarded_receiver_id: m.forwarded_receiver_id,
      forwarded_created_at: m.forwarded_created_at,
      forwarded_group_creator_id: m.forwarded_group_creator_id,
      forwarded_group_created_at: m.forwarded_group_created_at,
      is_pinned: m.is_pinned,
      pin_expired_at: m.pin_expired_at,
    }
  }
}

impl From<crate::entities::direct_chat::ActiveModel> for DirectChatResponse {
  fn from(m: crate::entities::direct_chat::ActiveModel) -> Self {
    Self {
      sender_id: m.sender_id.unwrap(),
      receiver_id: m.receiver_id.unwrap(),
      created_at: m.created_at.unwrap(),
      content: m.content.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      replied_sender_id: m.replied_sender_id.unwrap(),
      replied_receiver_id: m.replied_receiver_id.unwrap(),
      replied_created_at: m.replied_created_at.unwrap(),
      forwarded_sender_id: m.forwarded_sender_id.unwrap(),
      forwarded_receiver_id: m.forwarded_receiver_id.unwrap(),
      forwarded_created_at: m.forwarded_created_at.unwrap(),
      forwarded_group_creator_id: m.forwarded_group_creator_id.unwrap(),
      forwarded_group_created_at: m.forwarded_group_created_at.unwrap(),
      is_pinned: m.is_pinned.unwrap(),
      pin_expired_at: m.pin_expired_at.unwrap(),
    }
  }
}
