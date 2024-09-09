use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PersonAddRequest {
  pub id: Uuid,
  pub first_name: String,
  pub middle_name: Option<String>,
  pub last_name: Option<String>,
  pub dob: Option<Date>,
  pub sex: Option<bool>,
  pub deceased_at: Option<Date>,
  pub extra_info: Option<Json>,
  pub is_deceased: bool,
  pub photo: Option<String>,
  #[validate(email)]
  pub email: Option<String>,
  pub is_email_verified: bool,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PersonUpdateRequest {
  pub id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub first_name: String,
  pub middle_name: Option<String>,
  pub last_name: Option<String>,
  pub dob: Option<Date>,
  pub sex: Option<bool>,
  pub deceased_at: Option<Date>,
  pub extra_info: Option<Json>,
  pub is_deceased: bool,
  pub photo: Option<String>,
  #[validate(email)]
  pub email: Option<String>,
  pub is_email_verified: bool,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PersonResponse {
  pub id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub first_name: String,
  pub middle_name: Option<String>,
  pub last_name: Option<String>,
  pub dob: Option<Date>,
  pub sex: Option<bool>,
  pub deceased_at: Option<Date>,
  pub extra_info: Option<Json>,
  pub is_deceased: bool,
  pub photo: Option<String>,
  pub email: Option<String>,
  pub is_email_verified: bool,
}
