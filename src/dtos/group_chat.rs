use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct AddRequest {
  pub sender_id: Uuid,
  pub group_creator_id: Uuid,
  pub group_created_at: DateTimeWithTimeZone,
  pub content: String,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct Response {
  pub sender_id: Uuid,
  pub group_creator_id: Uuid,
  pub group_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  pub content: String,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}