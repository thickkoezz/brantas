use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct ProjectAddRequest {
  pub person_id: Uuid,
  pub name: String,
  pub description: Option<String>,
  pub start_date: Option<Date>,
  pub end_date: Option<Date>,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct ProjectUpdateRequest {
  pub person_id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub name: String,
  pub description: Option<String>,
  pub start_date: Option<Date>,
  pub end_date: Option<Date>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct ProjectResponse {
  pub person_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub name: String,
  pub description: Option<String>,
  pub start_date: Option<Date>,
  pub end_date: Option<Date>,
}

impl From<crate::entities::project::Model> for ProjectResponse {
  fn from(m: crate::entities::project::Model) -> Self {
    Self {
      person_id: m.person_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      name: m.name,
      description: m.description,
      start_date: m.start_date,
      end_date: m.end_date,
    }
  }
}

impl From<crate::entities::project::ActiveModel> for ProjectResponse {
  fn from(m: crate::entities::project::ActiveModel) -> Self {
    Self {
      person_id: m.person_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      name: m.name.unwrap(),
      description: m.description.unwrap(),
      start_date: m.start_date.unwrap(),
      end_date: m.end_date.unwrap(),
    }
  }
}
