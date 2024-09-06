use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone};
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
  pub view_count: i32,
  pub reply_count: i32,
  pub react_count: i32,
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
  pub reply_count: i32,
  pub react_count: i32,
  pub is_public: bool,
  pub group_name: Option<String>,
  pub can_comment: bool,
}