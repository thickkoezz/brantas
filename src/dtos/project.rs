use crate::entities::project::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone};
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct ProjectDTO {
  pub person_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start_date: Option<Date>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end_date: Option<Date>,
}

impl ProjectDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (self.person_id.clone(), self.created_at.clone())
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.person_id = v.0;
    self.created_at = v.1;
    self
  }

  pub fn set_person_id(&mut self, v: Uuid) -> &mut Self {
    self.person_id = v;
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

  pub fn set_name(&mut self, v: String) -> &mut Self {
    self.name = v;
    self
  }

  pub fn set_description(&mut self, v: Option<String>) -> &mut Self {
    self.description = v;
    self
  }

  pub fn set_start_date(&mut self, v: Option<Date>) -> &mut Self {
    self.start_date = v;
    self
  }

  pub fn set_end_date(&mut self, v: Option<Date>) -> &mut Self {
    self.end_date = v;
    self
  }
}

impl From<Model> for ProjectDTO {
  fn from(m: Model) -> Self {
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

impl From<ActiveModel> for ProjectDTO {
  fn from(m: ActiveModel) -> Self {
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
