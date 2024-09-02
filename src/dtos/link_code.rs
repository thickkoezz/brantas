use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct AddRequest {
  pub link_id: Uuid,
  pub code: String,
  pub expired_at: Option<DateTimeWithTimeZone>,
  pub is_public: bool,
  pub is_code_manually_typed: bool,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct Response {
  pub link_id: Uuid,
  pub code: String,
  pub created_at: DateTimeWithTimeZone,
  pub expired_at: Option<DateTimeWithTimeZone>,
  pub is_public: bool,
  pub is_code_manually_typed: bool,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}