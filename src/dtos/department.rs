use crate::entities::department::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Json};
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = Uuid;

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct DepartmentDTO {
  pub id: Uuid,
  pub organization_id: Uuid,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub abbreviation: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dob: Option<Date>,
  pub is_dead: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dead_at: Option<Date>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub extra_info: Option<Json>,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub logo: Option<String>,
}

impl DepartmentDTO {
  pub fn new() -> Self {
    Self {
      id: Uuid::new_v4(),
      created_at: DateTimeWithTimeZone::from(Local::now()),
      ..Default::default()
    }
  }

  pub fn create(
    organization_id: Uuid,
    name: String,
    abbreviation: Option<String>,
    description: Option<String>,
    dob: Option<Date>,
    is_dead: bool,
    dead_at: Option<Date>,
    extra_info: Option<Json>,
    parent_id: Option<Uuid>,
    logo: Option<String>,
  ) -> Self {
    Self {
      organization_id,
      name,
      abbreviation,
      description,
      dob,
      is_dead,
      dead_at,
      extra_info,
      parent_id,
      logo,
      ..Self::new()
    }
  }

  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    self.id.clone()
  }

  pub fn set_id(&mut self, v: Uuid) -> &mut Self {
    self.id = v;
    self
  }

  pub fn set_organization_id(&mut self, v: Uuid) -> &mut Self {
    self.organization_id = v;
    self
  }

  pub fn set_name(&mut self, v: String) -> &mut Self {
    self.name = v;
    self
  }

  pub fn set_abbreviation(&mut self, v: Option<String>) -> &mut Self {
    self.abbreviation = v;
    self
  }

  pub fn set_description(&mut self, v: Option<String>) -> &mut Self {
    self.description = v;
    self
  }

  pub fn set_dob(&mut self, v: Option<Date>) -> &mut Self {
    self.dob = v;
    self
  }

  pub fn set_is_dead(&mut self, v: bool) -> &mut Self {
    self.is_dead = v;
    self
  }

  pub fn set_dead_at(&mut self, v: Option<Date>) -> &mut Self {
    self.dead_at = v;
    self
  }

  pub fn set_extra_info(&mut self, v: Option<Json>) -> &mut Self {
    self.extra_info = v;
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

  pub fn set_parent_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.parent_id = v;
    self
  }

  pub fn set_logo(&mut self, v: Option<String>) -> &mut Self {
    self.logo = v;
    self
  }
}

impl From<Model> for DepartmentDTO {
  fn from(m: Model) -> Self {
    Self {
      id: m.id,
      organization_id: m.organization_id,
      name: m.name,
      abbreviation: m.abbreviation,
      description: m.description,
      dob: m.dob,
      is_dead: m.is_dead,
      dead_at: m.dead_at,
      extra_info: m.extra_info,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      parent_id: m.parent_id,
      logo: m.logo,
    }
  }
}

impl From<ActiveModel> for DepartmentDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      id: m.id.unwrap(),
      organization_id: m.organization_id.unwrap(),
      name: m.name.unwrap(),
      abbreviation: m.abbreviation.unwrap(),
      description: m.description.unwrap(),
      dob: m.dob.unwrap(),
      is_dead: m.is_dead.unwrap(),
      dead_at: m.dead_at.unwrap(),
      extra_info: m.extra_info.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      parent_id: m.parent_id.unwrap(),
      logo: m.logo.unwrap(),
    }
  }
}
