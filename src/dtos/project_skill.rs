use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct ProjectSkillAddRequest {
  pub person_id: Uuid,
  pub project_created_at: DateTimeWithTimeZone,
  pub skill_created_at: DateTimeWithTimeZone,
  pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct ProjectSkillResponse {
  pub person_id: Uuid,
  pub project_created_at: DateTimeWithTimeZone,
  pub skill_created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub description: Option<String>,
}

impl From<crate::entities::project_skill> for ProjectSkillResponse {
  fn from(m: crate::entities::project_skill) -> ProjectSkillResponse {
    ProjectSkillResponse {
      person_id: m.person_id,
      project_created_at: m.project_created_at,
      skill_created_at: m.skill_created_at,
      deleted_at: m.deleted_at,
      description: m.description,
    }
  }
}
