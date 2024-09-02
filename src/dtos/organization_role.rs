use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct AddRequest {
  pub organization_id: Uuid,
  pub name: String,
  pub description: Option<String>,
  pub extra_info: Option<Json>,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct UpdateRequest {
  pub organization_id: Uuid,
  pub name: String,
  pub description: Option<String>,
  pub extra_info: Option<Json>,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct Response {
  pub organization_id: Uuid,
  pub name: String,
  pub description: Option<String>,
  pub extra_info: Option<Json>,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}