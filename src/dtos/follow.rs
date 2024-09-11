use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct FollowAddRequest {
  pub follower_id: Uuid,
  pub target_id: Uuid,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct FollowResponse {
  pub follower_id: Uuid,
  pub target_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::follow> for FollowResponse {
  fn from(m: crate::entities::follow) -> FollowResponse {
    FollowResponse {
      follower_id: m.follower_id,
      target_id: m.target_id,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
    }
  }
}
