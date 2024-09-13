use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PostBookmarkAddRequest {
  pub owner_id: Uuid,
  pub bookmarked_post_owner_id: Uuid,
  pub bookmarked_post_created_at: DateTimeWithTimeZone,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PostBookmarkResponse {
  pub owner_id: Uuid,
  pub bookmarked_post_owner_id: Uuid,
  pub bookmarked_post_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
}

impl From<crate::entities::post_bookmark::Model> for PostBookmarkResponse {
  fn from(m: crate::entities::post_bookmark::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      bookmarked_post_owner_id: m.bookmarked_post_owner_id,
      bookmarked_post_created_at: m.bookmarked_post_created_at,
      created_at: m.created_at,
    }
  }
}

impl From<crate::entities::post_bookmark::ActiveModel> for PostBookmarkResponse {
  fn from(m: crate::entities::post_bookmark::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      bookmarked_post_owner_id: m.bookmarked_post_owner_id.unwrap(),
      bookmarked_post_created_at: m.bookmarked_post_created_at.unwrap(),
      created_at: m.created_at.unwrap(),
    }
  }
}
