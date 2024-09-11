use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct NoteEditHistoryAddRequest {
  pub note_owner_id: Uuid,
  pub note_created_at: DateTimeWithTimeZone,
  pub editor_id: Uuid,
  pub title: String,
  pub content: String,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct NoteEditHistoryResponse {
  pub note_owner_id: Uuid,
  pub note_created_at: DateTimeWithTimeZone,
  pub editor_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub title: String,
  pub content: String,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::note_edit_history> for NoteEditHistoryResponse {
  fn from(m: crate::entities::note_edit_history) -> NoteEditHistoryResponse {
    NoteEditHistoryResponse {
      note_owner_id: m.note_owner_id,
      note_created_at: m.note_created_at,
      editor_id: m.editor_id,
      created_at: m.created_at,
      title: m.title,
      content: m.content,
      deleted_at: m.deleted_at,
    }
  }
}
