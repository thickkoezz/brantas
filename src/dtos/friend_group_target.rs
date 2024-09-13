use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct FriendGroupTargetAddRequest {
  pub group_owner_id: Uuid,
  pub group_name: String,
  pub target_id: Uuid,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct FriendGroupTargetResponse {
  pub group_owner_id: Uuid,
  pub group_name: String,
  pub target_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::friend_group_target::Model> for FriendGroupTargetResponse {
  fn from(m: crate::entities::friend_group_target::Model) -> Self {
    Self {
      group_owner_id: m.group_owner_id,
      group_name: m.group_name,
      target_id: m.target_id,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
    }
  }
}
