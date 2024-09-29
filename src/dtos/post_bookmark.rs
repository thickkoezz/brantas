use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct PostBookmarkDTO {
  pub owner_id: Uuid,
  pub bookmarked_post_owner_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bookmarked_post_created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_at: DateTimeWithTimeZone,
}

impl PostBookmarkDTO {
  pub fn set_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.owner_id = v;
    self
  }

  pub fn set_bookmarked_post_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.bookmarked_post_owner_id = v;
    self
  }

  pub fn set_bookmarked_post_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.bookmarked_post_created_at = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }
}

impl From<crate::entities::post_bookmark::Model> for PostBookmarkDTO {
  fn from(m: crate::entities::post_bookmark::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      bookmarked_post_owner_id: m.bookmarked_post_owner_id,
      bookmarked_post_created_at: m.bookmarked_post_created_at,
      created_at: m.created_at,
    }
  }
}

impl From<crate::entities::post_bookmark::ActiveModel> for PostBookmarkDTO {
  fn from(m: crate::entities::post_bookmark::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      bookmarked_post_owner_id: m.bookmarked_post_owner_id.unwrap(),
      bookmarked_post_created_at: m.bookmarked_post_created_at.unwrap(),
      created_at: m.created_at.unwrap(),
    }
  }
}
