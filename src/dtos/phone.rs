use crate::entities::phone::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (String);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct PhoneDTO {
  pub phone: String,
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub is_verified: bool,
  pub is_suspended: bool,
}

impl PhoneDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    self.phone.clone()
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.phone = v;
    self
  }

  pub fn set_phone(&mut self, v: String) -> &mut Self {
    self.phone = v;
    self
  }

  pub fn set_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.owner_id = v;
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

  pub fn set_is_verified(&mut self, v: bool) -> &mut Self {
    self.is_verified = v;
    self
  }

  pub fn set_is_suspended(&mut self, v: bool) -> &mut Self {
    self.is_suspended = v;
    self
  }
}

impl From<Model> for PhoneDTO {
  fn from(m: Model) -> Self {
    Self {
      phone: m.phone,
      owner_id: m.owner_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      is_verified: m.is_verified,
      is_suspended: m.is_suspended,
    }
  }
}

impl From<ActiveModel> for PhoneDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      phone: m.phone.unwrap(),
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      is_verified: m.is_verified.unwrap(),
      is_suspended: m.is_suspended.unwrap(),
    }
  }
}
