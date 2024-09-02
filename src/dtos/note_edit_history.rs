use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct AddRequest {
  pub note_owner_id: Uuid,
  pub note_created_at: DateTimeWithTimeZone,
  pub editor_id: Uuid,
  pub title: String,
  pub content: String,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct Response {
  pub note_owner_id: Uuid,
  pub note_created_at: DateTimeWithTimeZone,
  pub editor_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub title: String,
  pub content: String,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}