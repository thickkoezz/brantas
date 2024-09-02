use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTime, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct AddRequest {
  pub group_creator_id: Uuid,
  pub group_created_at: DateTime,
  pub member_id: Uuid,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct UpdateRequest {
  pub group_creator_id: Uuid,
  pub group_created_at: DateTime,
  pub member_id: Uuid,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct Response {
  pub group_creator_id: Uuid,
  pub group_created_at: DateTime,
  pub member_id: Uuid,
  pub created_at: DateTime,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}