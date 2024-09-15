use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PostAddRequest {
  pub owner_id: Uuid,
  pub title: String,
  pub content: String,
  pub is_published: bool,
  pub hashtag: Option<String>,
  pub is_public: bool,
  pub group_name: Option<String>,
  pub can_comment: bool,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PostUpdateRequest {
  pub owner_id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: String,
  pub content: String,
  pub is_published: bool,
  pub hashtag: Option<String>,
  pub view_count: i32,
  pub reply_count: i32,
  pub react_count: i32,
  pub is_public: bool,
  pub group_name: Option<String>,
  pub can_comment: bool,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PostResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: String,
  pub content: String,
  pub is_published: bool,
  pub hashtag: Option<String>,
  pub view_count: i32,
  pub comment_count: i32,
  pub reaction_count: i32,
  pub is_public: bool,
  pub group_name: Option<String>,
  pub can_comment: bool,
}

impl From<crate::entities::post::Model> for PostResponse {
  fn from(m: crate::entities::post::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      title: m.title,
      content: m.content,
      is_published: m.is_published,
      hashtag: m.hashtag,
      view_count: m.view_count,
      comment_count: m.comment_count,
      reaction_count: m.reaction_count,
      is_public: m.is_public,
      group_name: m.group_name,
      can_comment: m.can_comment,
    }
  }
}

impl From<crate::entities::post::ActiveModel> for PostResponse {
  fn from(m: crate::entities::post::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      title: m.title.unwrap(),
      content: m.content.unwrap(),
      is_published: m.is_published.unwrap(),
      hashtag: m.hashtag.unwrap(),
      view_count: m.view_count.unwrap(),
      comment_count: m.comment_count.unwrap(),
      reaction_count: m.reaction_count.unwrap(),
      is_public: m.is_public.unwrap(),
      group_name: m.group_name.unwrap(),
      can_comment: m.can_comment.unwrap(),
    }
  }
}
