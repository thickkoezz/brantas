use crate::entities::project_skill::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct ProjectSkillDTO {
  pub person_id: Uuid,
  pub project_created_at: DateTimeWithTimeZone,
  pub skill_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}

impl ProjectSkillDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (
      self.person_id.clone(),
      self.project_created_at.clone(),
      self.skill_created_at.clone(),
    )
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.person_id = v.0;
    self.project_created_at = v.1;
    self.skill_created_at = v.2;
    self
  }

  pub fn set_person_id(&mut self, v: Uuid) -> &mut Self {
    self.person_id = v;
    self
  }

  pub fn set_project_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.project_created_at = v;
    self
  }

  pub fn set_skill_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.skill_created_at = v;
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

impl From<Model> for ProjectSkillDTO {
  fn from(m: Model) -> Self {
    Self {
      person_id: m.person_id,
      project_created_at: m.project_created_at,
      skill_created_at: m.skill_created_at,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
      description: m.description,
    }
  }
}

impl From<ActiveModel> for ProjectSkillDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      person_id: m.person_id.unwrap(),
      project_created_at: m.project_created_at.unwrap(),
      skill_created_at: m.skill_created_at.unwrap(),
      created_at: m.created_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      description: m.description.unwrap(),
    }
  }
}
