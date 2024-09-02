use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct AddRequest {
  pub organization_id: Uuid,
  pub administrator_id: Uuid,
  pub department_id: Uuid,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct Response {
  pub organization_id: Uuid,
  pub administrator_id: Uuid,
  pub department_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}