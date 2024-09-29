use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone};
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct JobDTO {
  pub organization_id: Uuid,
  pub person_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub department_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub role: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub job_description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start_at: Option<Date>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end_at: Option<Date>,
  pub is_head_of_department: bool,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl JobDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(chrono::Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (self.organization_id.clone(), self.person_id.clone())
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.organization_id = v.0;
    self.person_id = v.1;
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

  pub fn set_department_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.department_id = v;
    self
  }

  pub fn set_role(&mut self, v: Option<String>) -> &mut Self {
    self.role = v;
    self
  }

  pub fn set_job_description(&mut self, v: Option<String>) -> &mut Self {
    self.job_description = v;
    self
  }

  pub fn set_start_at(&mut self, v: Option<Date>) -> &mut Self {
    self.start_at = v;
    self
  }

  pub fn set_end_at(&mut self, v: Option<Date>) -> &mut Self {
    self.end_at = v;
    self
  }

  pub fn set_is_head_of_department(&mut self, v: bool) -> &mut Self {
    self.is_head_of_department = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_updated_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.updated_at = v;
    self
  }

  pub fn set_deleted_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.deleted_at = v;
    self
  }
}

impl From<crate::entities::job::Model> for JobDTO {
  fn from(m: crate::entities::job::Model) -> Self {
    Self {
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

impl From<crate::entities::job::ActiveModel> for JobDTO {
  fn from(m: crate::entities::job::ActiveModel) -> Self {
    Self {
      organization_id: m.organization_id.unwrap(),
      person_id: m.person_id.unwrap(),
      department_id: m.department_id.unwrap(),
      role: m.role.unwrap(),
      job_description: m.job_description.unwrap(),
      start_at: m.start_at.unwrap(),
      end_at: m.end_at.unwrap(),
      is_head_of_department: m.is_head_of_department.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
