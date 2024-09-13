use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct EmailAddRequest {
  pub email: String,
  pub owner_id: Uuid,
  pub is_verified: bool,
  pub is_suspended: bool,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct EmailUpdateRequest {
  pub email: String,
  pub owner_id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub is_verified: bool,
  pub is_suspended: bool,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct EmailResponse {
  pub email: String,
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub is_verified: bool,
  pub is_suspended: bool,
}

impl From<crate::entities::email::Model> for EmailResponse {
  fn from(m: crate::entities::email::Model) -> Self {
    Self {
      email: m.email,
      owner_id: m.owner_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      is_verified: m.is_verified,
      is_suspended: m.is_suspended,
    }
  }
}
