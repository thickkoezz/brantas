use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone, Decimal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct AddRequest {
  pub owner_id: Uuid,
  pub ref_id: Uuid,
  pub amount: Decimal,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct UpdateRequest {
  pub owner_id: Uuid,
  pub ref_id: Uuid,
  pub amount: Decimal,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct Response {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub ref_id: Uuid,
  pub amount: Decimal,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}