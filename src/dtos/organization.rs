use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct OrganizationAddRequest {
  pub id: Uuid,
  pub name: String,
  pub abbreviation: Option<String>,
  pub description: Option<String>,
  pub dob: Option<Date>,
  pub dead_at: Option<Date>,
  pub extra_info: Option<Json>,
  pub is_dead: bool,
  pub parent_id: Option<Uuid>,
  pub logo: Option<String>,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct OrganizationUpdateRequest {
  pub id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub name: String,
  pub abbreviation: Option<String>,
  pub description: Option<String>,
  pub dob: Option<Date>,
  pub dead_at: Option<Date>,
  pub extra_info: Option<Json>,
  pub is_dead: bool,
  pub parent_id: Option<Uuid>,
  pub logo: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct OrganizationResponse {
  pub id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub name: String,
  pub abbreviation: Option<String>,
  pub description: Option<String>,
  pub dob: Option<Date>,
  pub dead_at: Option<Date>,
  pub extra_info: Option<Json>,
  pub is_dead: bool,
  pub parent_id: Option<Uuid>,
  pub logo: Option<String>,
}
