use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PhotoCommentAddRequest {
  pub owner_id: Uuid,
  pub commented_photo_owner_id: Uuid,
  pub commented_photo_created_at: DateTimeWithTimeZone,
  pub content: String,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PhotoCommentUpdateRequest {
  pub owner_id: Uuid,
  pub commented_photo_owner_id: Uuid,
  pub commented_photo_created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub content: String,
  pub reaction_count: i32,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PhotoCommentResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub commented_photo_owner_id: Uuid,
  pub commented_photo_created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub content: String,
  pub reaction_count: i32,
}

impl From<crate::entities::photo_comment::Model> for PhotoCommentResponse {
  fn from(m: crate::entities::photo_comment::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      commented_photo_owner_id: m.commented_photo_owner_id,
      commented_photo_created_at: m.commented_photo_created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      content: m.content,
      reaction_count: m.reaction_count,
    }
  }
}

impl From<crate::entities::photo_comment::ActiveModel> for PhotoCommentResponse {
  fn from(m: crate::entities::photo_comment::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      commented_photo_owner_id: m.commented_photo_owner_id.unwrap(),
      commented_photo_created_at: m.commented_photo_created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      content: m.content.unwrap(),
      reaction_count: m.reaction_count.unwrap(),
    }
  }
}
