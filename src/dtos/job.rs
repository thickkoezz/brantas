use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct JobAddRequest {
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
pub struct JobUpdateRequest {
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
pub struct JobResponse {
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

impl From<crate::entities::job> for JobResponse {
  fn from(m: crate::entities::job) -> JobResponse {
    JobResponse {
      organization_id: m.organization_id,
      person_id: m.person_id,
      department_id: m.department_id,
      role: m.role,
      job_description: m.job_description,
      start_at: m.start_at,
      end_at: m.end_at,
      is_head_of_department: m.is_head_of_department,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
    }
  }
}
