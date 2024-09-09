use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PhotoAddRequest {
  pub owner_id: Uuid,
  pub photo: String,
  pub size: i32,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PhotoUpdateRequest {
  pub owner_id: Uuid,
  pub photo: String,
  pub size: i32,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PhotoResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub photo: String,
  pub size: i32,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}
