use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct VideoCommentAddRequest {
  pub owner_id: Uuid,
  pub commented_video_owner_id: Uuid,
  pub commented_video_created_at: DateTimeWithTimeZone,
  pub content: String,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct VideoCommentUpdateRequest {
  pub owner_id: Uuid,
  pub commented_video_owner_id: Uuid,
  pub commented_video_created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub content: String,
  pub reaction_count: i32,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct VideoCommentResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub commented_video_owner_id: Uuid,
  pub commented_video_created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub content: String,
  pub reaction_count: i32,
}

impl From<crate::entities::video_comment::Model> for VideoCommentResponse {
  fn from(m: crate::entities::video_comment::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      commented_video_owner_id: m.commented_video_owner_id,
      commented_video_created_at: m.commented_video_created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      content: m.content,
      reaction_count: m.reaction_count,
    }
  }
}

impl From<crate::entities::video_comment::ActiveModel> for VideoCommentResponse {
  fn from(m: crate::entities::video_comment::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      commented_video_owner_id: m.commented_video_owner_id.unwrap(),
      commented_video_created_at: m.commented_video_created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      content: m.content.unwrap(),
      reaction_count: m.reaction_count.unwrap(),
    }
  }
}
