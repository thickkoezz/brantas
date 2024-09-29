use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone, Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct NoteEditHistoryDTO {
  pub note_owner_id: Uuid,
  pub note_created_at: DateTimeWithTimeZone,
  pub editor_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub title: String,
  pub content: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl NoteEditHistoryDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(chrono::Local::now()));
    self
  }

  pub fn set_note_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.note_owner_id = v;
    self
  }

  pub fn set_note_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.note_created_at = v;
    self
  }

  pub fn set_editor_id(&mut self, v: Uuid) -> &mut Self {
    self.editor_id = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_title(&mut self, v: String) -> &mut Self {
    self.title = v;
    self
  }

  pub fn set_content(&mut self, v: String) -> &mut Self {
    self.content = v;
    self
  }

  pub fn set_deleted_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.deleted_at = v;
    self
  }
}

impl From<crate::entities::note_edit_history::Model> for NoteEditHistoryDTO {
  fn from(m: crate::entities::note_edit_history::Model) -> Self {
    Self {
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

impl From<crate::entities::note_edit_history::ActiveModel> for NoteEditHistoryDTO {
  fn from(m: crate::entities::note_edit_history::ActiveModel) -> Self {
    Self {
      note_owner_id: m.note_owner_id.unwrap(),
      note_created_at: m.note_created_at.unwrap(),
      editor_id: m.editor_id.unwrap(),
      created_at: m.created_at.unwrap(),
      title: m.title.unwrap(),
      content: m.content.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
