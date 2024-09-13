use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct MessageAddRequest {
  pub owner_id: Uuid,
  pub receiver_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub message: String,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct MessageUpdateRequest {
  pub owner_id: Uuid,
  pub receiver_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub message: String,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct MessageResponse {
  pub owner_id: Uuid,
  pub receiver_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub message: String,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::message::Model> for MessageResponse {
  fn from(m: crate::entities::message::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      receiver_id: m.receiver_id,
      created_at: m.created_at,
      message: m.message,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
    }
  }
}
