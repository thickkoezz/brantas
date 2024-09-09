use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTime, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PostShareAddRequest {
  pub post_owner_id: Uuid,
  pub post_created_at: DateTime,
  pub target_id: Uuid,
  pub can_comment: bool,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PostShareUpdateRequest {
  pub post_owner_id: Uuid,
  pub post_created_at: DateTime,
  pub target_id: Uuid,
  pub can_comment: bool,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PostShareResponse {
  pub post_owner_id: Uuid,
  pub post_created_at: DateTime,
  pub target_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub can_comment: bool,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}
