use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct LinkCodeAddRequest {
  pub link_id: Uuid,
  pub code: String,
  pub expired_at: Option<DateTimeWithTimeZone>,
  pub is_public: bool,
  pub is_code_manually_typed: bool,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct LinkCodeResponse {
  pub link_id: Uuid,
  pub code: String,
  pub created_at: DateTimeWithTimeZone,
  pub expired_at: Option<DateTimeWithTimeZone>,
  pub is_public: bool,
  pub is_code_manually_typed: bool,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::link_code::Model> for LinkCodeResponse {
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
