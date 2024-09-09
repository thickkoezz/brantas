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
