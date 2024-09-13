use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct SocmedUrlAddRequest {
  #[validate(url)]
  pub socmed_url: String,
  pub owner_id: Uuid,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct SocmedUrlUpdateRequest {
  pub socmed_url: String,
  pub owner_id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct SocmedUrlResponse {
  pub socmed_url: String,
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::socmed_url::Model> for SocmedUrlResponse {
  fn from(m: crate::entities::socmed_url::Model) -> Self {
    Self {
      socmed_url: m.socmed_url,
      owner_id: m.owner_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<crate::entities::socmed_url::ActiveModel> for SocmedUrlResponse {
  fn from(m: crate::entities::socmed_url::ActiveModel) -> Self {
    Self {
      socmed_url: m.socmed_url.unwrap(),
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
