use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = Uuid;

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct LinkCodeDTO {
  pub link_id: Uuid,
  pub code: String,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expired_at: Option<DateTimeWithTimeZone>,
  pub is_public: bool,
  pub is_code_manually_typed: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl LinkCodeDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(chrono::Local::now()));
    self
  }

  pub fn set_link_id(&mut self, v: Uuid) -> &mut Self {
    self.link_id = v;
    self
  }

  pub fn set_code(&mut self, v: String) -> &mut Self {
    self.code = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_expired_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.expired_at = v;
    self
  }

  pub fn set_is_public(&mut self, v: bool) -> &mut Self {
    self.is_public = v;
    self
  }

  pub fn set_is_code_manually_typed(&mut self, v: bool) -> &mut Self {
    self.is_code_manually_typed = v;
    self
  }

  pub fn set_deleted_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.deleted_at = v;
    self
  }
}

impl From<crate::entities::link_code::Model> for LinkCodeDTO {
  fn from(m: crate::entities::link_code::Model) -> Self {
    Self {
      link_id: m.link_id,
      code: m.code,
      created_at: m.created_at,
      expired_at: m.expired_at,
      is_public: m.is_public,
      is_code_manually_typed: m.is_code_manually_typed,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<crate::entities::link_code::ActiveModel> for LinkCodeDTO {
  fn from(m: crate::entities::link_code::ActiveModel) -> Self {
    Self {
      link_id: m.link_id.unwrap(),
      code: m.code.unwrap(),
      created_at: m.created_at.unwrap(),
      expired_at: m.expired_at.unwrap(),
      is_public: m.is_public.unwrap(),
      is_code_manually_typed: m.is_code_manually_typed.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
