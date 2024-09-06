use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct JobProjectAddRequest {
  pub organization_id: Uuid,
  pub person_id: Uuid,
  pub job_created_at: DateTimeWithTimeZone,
  pub project_created_at: DateTimeWithTimeZone,
  pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct JobProjectResponse {
  pub organization_id: Uuid,
  pub person_id: Uuid,
  pub job_created_at: DateTimeWithTimeZone,
  pub project_created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub description: Option<String>,
}