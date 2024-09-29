use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct VideoDTO {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub video: String,
  pub size: i32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub title: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub caption: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub code: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub slug: Option<String>,
  pub is_private: bool,
}

impl VideoDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(chrono::Local::now()));
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

  pub fn set_video(&mut self, v: String) -> &mut Self {
    self.video = v;
    self
  }

  pub fn set_size(&mut self, v: i32) -> &mut Self {
    self.size = v;
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

  pub fn set_title(&mut self, v: Option<String>) -> &mut Self {
    self.title = v;
    self
  }

  pub fn set_caption(&mut self, v: Option<String>) -> &mut Self {
    self.caption = v;
    self
  }

  pub fn set_code(&mut self, v: Option<String>) -> &mut Self {
    self.code = v;
    self
  }

  pub fn set_slug(&mut self, v: Option<String>) -> &mut Self {
    self.slug = v;
    self
  }

  pub fn set_is_private(&mut self, v: bool) -> &mut Self {
    self.is_private = v;
    self
  }
}

impl From<crate::entities::video::Model> for VideoDTO {
  fn from(m: crate::entities::video::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      video: m.video,
      size: m.size,
      deleted_at: m.deleted_at,
      title: m.title,
      caption: m.caption,
      code: m.code,
      slug: m.slug,
      is_private: m.is_private,
    }
  }
}

impl From<crate::entities::video::ActiveModel> for VideoDTO {
  fn from(m: crate::entities::video::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      video: m.video.unwrap(),
      size: m.size.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      title: m.title.unwrap(),
      caption: m.caption.unwrap(),
      code: m.code.unwrap(),
      slug: m.slug.unwrap(),
      is_private: m.is_private.unwrap(),
    }
  }
}
