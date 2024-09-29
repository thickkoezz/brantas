use crate::entities::organization_address::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, i32);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct OrganizationAddressDTO {
  pub organization_id: Uuid,
  pub city_id: i32,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}

impl OrganizationAddressDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (self.organization_id.clone(), self.city_id.clone())
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.organization_id = v.0;
    self.city_id = v.1;
    self
  }

  pub fn set_organization_id(&mut self, v: Uuid) -> &mut Self {
    self.organization_id = v;
    self
  }

  pub fn set_city_id(&mut self, v: i32) -> &mut Self {
    self.city_id = v;
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

  pub fn set_description(&mut self, v: Option<String>) -> &mut Self {
    self.description = v;
    self
  }
}

impl From<Model> for OrganizationAddressDTO {
  fn from(m: Model) -> Self {
    Self {
      organization_id: m.organization_id,
      city_id: m.city_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      description: m.description,
    }
  }
}

impl From<ActiveModel> for OrganizationAddressDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      organization_id: m.organization_id.unwrap(),
      city_id: m.city_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      description: m.description.unwrap(),
    }
  }
}
