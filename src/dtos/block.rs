use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct BlockAddRequest {
  pub blocker_id: Uuid,
  pub target_id: Uuid,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct BlockUpdateRequest {
  pub blocker_id: Uuid,
  pub target_id: Uuid,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct BlockResponse {
  pub blocker_id: Uuid,
  pub target_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}