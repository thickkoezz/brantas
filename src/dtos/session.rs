use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct AddRequest {
  pub id: Uuid,
  pub session_token: String,
  pub user_account_id: Uuid,
  pub expires: DateTimeWithTimeZone,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct Response {
  pub id: Uuid,
  pub session_token: String,
  pub user_account_id: Uuid,
  pub expires: DateTimeWithTimeZone,
}