use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PostCommentAddRequest {
  pub owner_id: Uuid,
  pub commented_post_owner_id: Uuid,
  pub commented_post_created_at: DateTimeWithTimeZone,
  pub content: String,
  pub react_count: i32,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PostCommentUpdateRequest {
  pub owner_id: Uuid,
  pub commented_post_owner_id: Uuid,
  pub commented_post_created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub content: String,
  pub react_count: i32,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PostCommentResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub commented_post_owner_id: Uuid,
  pub commented_post_created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub content: String,
  pub react_count: i32,
}