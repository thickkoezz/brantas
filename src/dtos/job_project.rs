use crate::entities::job_project::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid, DateTimeWithTimeZone, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct JobProjectDTO {
  pub organization_id: Uuid,
  pub person_id: Uuid,
  pub job_created_at: DateTimeWithTimeZone,
  pub project_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}

impl JobProjectDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (
      self.organization_id.clone(),
      self.person_id.clone(),
      self.job_created_at.clone(),
      self.project_created_at.clone(),
    )
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.organization_id = v.0;
    self.person_id = v.1;
    self.job_created_at = v.2;
    self.project_created_at = v.3;
    self
  }

  pub fn set_organization_id(&mut self, v: Uuid) -> &mut Self {
    self.organization_id = v;
    self
  }

  pub fn set_person_id(&mut self, v: Uuid) -> &mut Self {
    self.person_id = v;
    self
  }

  pub fn set_job_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.job_created_at = v;
    self
  }

  pub fn set_project_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.project_created_at = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_deleted_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.deleted_at = v;
    self
  }

  pub fn set_description(&mut self, v: Option<String>) -> &mut Self {
    self.description = v;
    self
  }
}

impl From<Model> for JobProjectDTO {
  fn from(m: Model) -> Self {
    Self {
      organization_id: m.organization_id,
      person_id: m.person_id,
      job_created_at: m.job_created_at,
      project_created_at: m.project_created_at,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
      description: m.description,
    }
  }
}

impl From<ActiveModel> for JobProjectDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      organization_id: m.organization_id.unwrap(),
      person_id: m.person_id.unwrap(),
      job_created_at: m.job_created_at.unwrap(),
      project_created_at: m.project_created_at.unwrap(),
      created_at: m.created_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      description: m.description.unwrap(),
    }
  }
}
