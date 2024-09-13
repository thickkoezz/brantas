use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
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

impl From<crate::entities::job_project::Model> for Self {
  fn from(m: crate::entities::job_project::Model) -> JobProjectResponse {
    Self {
      organization_id: m.organization_id,
      person_id: m.person_id,
      job_created_at: m.job_created_at,
      project_created_at: m.project_created_at,
      deleted_at: m.deleted_at,
      description: m.description,
    }
  }
}
