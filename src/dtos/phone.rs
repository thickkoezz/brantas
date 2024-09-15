use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PhoneAddRequest {
  pub phone: String,
  pub owner_id: Uuid,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PhoneUpdateRequest {
  pub phone: String,
  pub owner_id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub is_verified: bool,
  pub is_suspended: bool,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PhoneResponse {
  pub phone: String,
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub is_verified: bool,
  pub is_suspended: bool,
}

impl From<crate::entities::phone::Model> for PhoneResponse {
  fn from(m: crate::entities::phone::Model) -> Self {
    Self {
      phone: m.phone,
      owner_id: m.owner_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      is_verified: m.is_verified,
      is_suspended: m.is_suspended,
    }
  }
}

impl From<crate::entities::phone::ActiveModel> for PhoneResponse {
  fn from(m: crate::entities::phone::ActiveModel) -> Self {
    Self {
      phone: m.phone.unwrap(),
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      is_verified: m.is_verified.unwrap(),
      is_suspended: m.is_suspended.unwrap(),
    }
  }
}
