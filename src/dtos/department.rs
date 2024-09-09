use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct DepartmentAddRequest {
  pub id: Uuid,
  pub organization_id: Uuid,
  pub name: String,
  pub abbreviation: Option<String>,
  pub description: Option<String>,
  pub dob: Option<Date>,
  pub is_dead: bool,
  pub dead_at: Option<Date>,
  pub extra_info: Option<Json>,
  pub parent_id: Option<Uuid>,
  pub logo: Option<String>,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct DepartmentUpdateRequest {
  pub id: Uuid,
  pub organization_id: Uuid,
  pub name: String,
  pub abbreviation: Option<String>,
  pub description: Option<String>,
  pub dob: Option<Date>,
  pub is_dead: bool,
  pub dead_at: Option<Date>,
  pub extra_info: Option<Json>,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub parent_id: Option<Uuid>,
  pub logo: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct DepartmentResponse {
  pub id: Uuid,
  pub organization_id: Uuid,
  pub name: String,
  pub abbreviation: Option<String>,
  pub description: Option<String>,
  pub dob: Option<Date>,
  pub is_dead: bool,
  pub dead_at: Option<Date>,
  pub extra_info: Option<Json>,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub parent_id: Option<Uuid>,
  pub logo: Option<String>,
}
