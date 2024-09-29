use salvo::macros::Extractible;
use salvo::oapi::ToSchema;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = Uuid;

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct LinkDTO {
  pub id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub owner_id: Option<Uuid>,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub link_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub hashtag: Option<String>,
  pub use_count: i32,
}

impl LinkDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(chrono::Local::now()));
    self
  }

  pub fn set_id(&mut self, v: Uuid) -> &mut Self {
    self.id = v;
    self
  }

  pub fn set_owner_id(&mut self, v: Option<Uuid>) -> &mut Self {
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

  pub fn set_link_url(&mut self, v: String) -> &mut Self {
    self.link_url = v;
    self
  }

  pub fn set_hashtag(&mut self, v: Option<String>) -> &mut Self {
    self.hashtag = v;
    self
  }

  pub fn set_use_count(&mut self, v: i32) -> &mut Self {
    self.use_count = v;
    self
  }
}

impl From<crate::entities::link::Model> for LinkDTO {
  fn from(m: crate::entities::link::Model) -> Self {
    Self {
      id: m.id,
      owner_id: m.owner_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      link_url: m.link_url,
      hashtag: m.hashtag,
      use_count: m.use_count,
    }
  }
}

impl From<crate::entities::link::ActiveModel> for LinkDTO {
  fn from(m: crate::entities::link::ActiveModel) -> Self {
    Self {
      id: m.id.unwrap(),
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      link_url: m.link_url.unwrap(),
      hashtag: m.hashtag.unwrap(),
      use_count: m.use_count.unwrap(),
    }
  }
}
