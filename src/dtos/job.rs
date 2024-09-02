use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct AddRequest {
  pub organization_id: Uuid,
  pub person_id: Uuid,
  pub department_id: Option<Uuid>,
  pub role: Option<String>,
  pub job_description: Option<String>,
  pub start_at: Option<Date>,
  pub end_at: Option<Date>,
  pub is_head_of_department: bool,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct UpdateRequest {
  pub organization_id: Uuid,
  pub person_id: Uuid,
  pub department_id: Option<Uuid>,
  pub role: Option<String>,
  pub job_description: Option<String>,
  pub start_at: Option<Date>,
  pub end_at: Option<Date>,
  pub is_head_of_department: bool,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct Response {
  pub organization_id: Uuid,
  pub person_id: Uuid,
  pub department_id: Option<Uuid>,
  pub role: Option<String>,
  pub job_description: Option<String>,
  pub start_at: Option<Date>,
  pub end_at: Option<Date>,
  pub is_head_of_department: bool,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}