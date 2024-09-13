use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct NoteAddRequest {
  pub owner_id: Uuid,
  pub title: String,
  pub content: String,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct NoteUpdateRequest {
  pub owner_id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: String,
  pub content: String,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct NoteResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: String,
  pub content: String,
}

impl From<crate::entities::note::Model> for NoteResponse {
  fn from(m: crate::entities::note::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      title: m.title,
      content: m.content,
    }
  }
}

impl From<crate::entities::note::ActiveModel> for NoteResponse {
  fn from(m: crate::entities::note::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      title: m.title.unwrap(),
      content: m.content.unwrap(),
    }
  }
}
