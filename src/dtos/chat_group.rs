use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct ChatGroupAddRequest {
  pub creator_id: Uuid,
  pub name: Option<String>,
  pub is_public: bool,
  pub is_suspended: bool,
  pub is_channel: bool,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct ChatGroupUpdateRequest {
  pub creator_id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub name: Option<String>,
  pub is_public: bool,
  pub is_suspended: bool,
  pub is_channel: bool,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct ChatGroupResponse {
  pub creator_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub name: Option<String>,
  pub is_public: bool,
  pub is_suspended: bool,
  pub is_channel: bool,
}

impl From<crate::entities::chat_group::Model> for ChatGroupResponse {
  fn from(m: crate::entities::chat_group::Model) -> Self {
    Self {
      creator_id: m.creator_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      name: m.name,
      is_public: m.is_public,
      is_suspended: m.is_suspended,
      is_channel: m.is_channel,
    }
  }
}
