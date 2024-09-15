use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct AddRequest {
  pub blocker_id: Uuid,
  pub target_id: Uuid,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct UpdateRequest {
  pub blocker_id: Uuid,
  pub target_id: Uuid,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct Response {
  pub blocker_id: Uuid,
  pub target_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::block::Model> for Response {
  fn from(m: crate::entities::block::Model) -> Self {
    Self {
      blocker_id: m.blocker_id,
      target_id: m.target_id,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<crate::entities::block::ActiveModel> for Response {
  fn from(m: crate::entities::block::ActiveModel) -> Self {
    Self {
      blocker_id: m.blocker_id.unwrap(),
      target_id: m.target_id.unwrap(),
      created_at: m.created_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
