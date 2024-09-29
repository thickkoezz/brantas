use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid, Uuid);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct OrganizationAdministratorDTO {
  pub organization_id: Uuid,
  pub administrator_id: Uuid,
  pub department_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl OrganizationAdministratorDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(chrono::Local::now()));
    self
  }

  pub fn set_organization_id(&mut self, v: Uuid) -> &mut Self {
    self.organization_id = v;
    self
  }

  pub fn set_administrator_id(&mut self, v: Uuid) -> &mut Self {
    self.administrator_id = v;
    self
  }

  pub fn set_department_id(&mut self, v: Uuid) -> &mut Self {
    self.department_id = v;
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
}

impl From<crate::entities::organization_administrator::Model> for OrganizationAdministratorDTO {
  fn from(m: crate::entities::organization_administrator::Model) -> Self {
    Self {
      organization_id: m.organization_id,
      administrator_id: m.administrator_id,
      department_id: m.department_id,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<crate::entities::organization_administrator::ActiveModel>
  for OrganizationAdministratorDTO
{
  fn from(m: crate::entities::organization_administrator::ActiveModel) -> Self {
    Self {
      organization_id: m.organization_id.unwrap(),
      administrator_id: m.administrator_id.unwrap(),
      department_id: m.department_id.unwrap(),
      created_at: m.created_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
