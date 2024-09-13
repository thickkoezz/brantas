use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTime, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct ChatGroupMemberAddRequest {
  pub group_creator_id: Uuid,
  pub group_created_at: DateTime,
  pub member_id: Uuid,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct ChatGroupMemberUpdateRequest {
  pub group_creator_id: Uuid,
  pub group_created_at: DateTime,
  pub member_id: Uuid,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct ChatGroupMemberResponse {
  pub group_creator_id: Uuid,
  pub group_created_at: DateTimeWithTimeZone,
  pub member_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::chat_group_member::Model> for ChatGroupMemberResponse {
  fn from(m: crate::entities::chat_group_member::Model) -> Self {
    Self {
      group_creator_id: m.group_creator_id,
      group_created_at: m.group_created_at,
      member_id: m.member_id,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
    }
  }
}
