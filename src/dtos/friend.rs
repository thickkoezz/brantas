use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct FriendAddRequest {
  pub invitor_id: Uuid,
  pub invitee_id: Uuid,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct FriendResponse {
  pub invitor_id: Uuid,
  pub invitee_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}