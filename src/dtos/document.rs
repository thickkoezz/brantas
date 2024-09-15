use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct DocumentAddRequest {
  pub owner_id: Uuid,
  pub document: String,
  pub size: i32,
  pub title: Option<String>,
  pub caption: Option<String>,
  pub code: Option<String>,
  pub slug: Option<String>,
  pub is_private: bool,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct DocumentUpdateRequest {
  pub owner_id: Uuid,
  pub document: String,
  pub size: i32,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: Option<String>,
  pub caption: Option<String>,
  pub code: Option<String>,
  pub slug: Option<String>,
  pub is_private: bool,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct DocumentResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub document: String,
  pub size: i32,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: Option<String>,
  pub caption: Option<String>,
  pub code: Option<String>,
  pub slug: Option<String>,
  pub is_private: bool,
}

impl From<crate::entities::document::Model> for DocumentResponse {
  fn from(m: crate::entities::document::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      document: m.document,
      size: m.size,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      title: m.title,
      caption: m.caption,
      code: m.code,
      slug: m.slug,
      is_private: m.is_private,
    }
  }
}

impl From<crate::entities::document::ActiveModel> for DocumentResponse {
  fn from(m: crate::entities::document::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      document: m.document.unwrap(),
      size: m.size.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      title: m.title.unwrap(),
      caption: m.caption.unwrap(),
      code: m.code.unwrap(),
      slug: m.slug.unwrap(),
      is_private: m.is_private.unwrap(),
    }
  }
}
