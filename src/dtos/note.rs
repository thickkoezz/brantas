use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct NoteAddRequest {
  pub owner_id: Uuid,
  pub title: String,
  pub content: String,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct NoteUpdateRequest {
  pub owner_id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: String,
  pub content: String,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct NoteResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: String,
  pub content: String,
}