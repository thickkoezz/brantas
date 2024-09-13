use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct FriendGroupAddRequest {
  pub owner_id: Uuid,
  pub name: String,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct FriendGroupUpdateRequest {
  pub owner_id: Uuid,
  pub name: String,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct FriendGroupResponse {
  pub owner_id: Uuid,
  pub name: String,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::friend_group::Model> for FriendGroupResponse {
  fn from(m: crate::entities::friend_group::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      name: m.name,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<crate::entities::friend_group::ActiveModel> for FriendGroupResponse {
  fn from(m: crate::entities::friend_group::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      name: m.name.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
