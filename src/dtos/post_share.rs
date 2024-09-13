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
  pub post_created_at: DateTimeWithTimeZone,
  pub target_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub can_comment: bool,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::post_share::Model> for PostShareResponse {
  fn from(m: crate::entities::post_share::Model) -> Self {
    Self {
      post_owner_id: m.post_owner_id,
      post_created_at: m.post_created_at,
      target_id: m.target_id,
      created_at: m.created_at,
      can_comment: m.can_comment,
      deleted_at: m.deleted_at,
    }
  }
}
