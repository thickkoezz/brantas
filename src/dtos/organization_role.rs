use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone, Json};
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, String);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct OrganizationRoleDTO {
  pub organization_id: Uuid,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub extra_info: Option<Json>,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl OrganizationRoleDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(chrono::Local::now()));
    self
  }

  pub fn set_organization_id(&mut self, v: Uuid) -> &mut Self {
    self.organization_id = v;
    self
  }

  pub fn set_name(&mut self, name: String) -> &mut Self {
    self.name = name;
    self
  }

  pub fn set_description(&mut self, v: Option<String>) -> &mut Self {
    self.description = v;
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
}

impl From<crate::entities::organization_role::Model> for OrganizationRoleDTO {
  fn from(m: crate::entities::organization_role::Model) -> Self {
    Self {
      organization_id: m.organization_id,
      name: m.name,
      description: m.description,
      extra_info: m.extra_info,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<crate::entities::organization_role::ActiveModel> for OrganizationRoleDTO {
  fn from(m: crate::entities::organization_role::ActiveModel) -> Self {
    Self {
      organization_id: m.organization_id.unwrap(),
      name: m.name.unwrap(),
      description: m.description.unwrap(),
      extra_info: m.extra_info.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
