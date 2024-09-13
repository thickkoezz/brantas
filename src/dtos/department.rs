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

impl From<crate::entities::department::Model> for DepartmentResponse {
  fn from(m: crate::entities::department::Model) -> Self {
    Self {
      id: m.id,
      organization_id: m.organization_id,
      name: m.name,
      abbreviation: m.abbreviation,
      description: m.description,
      dob: m.dob,
      is_dead: m.is_dead,
      dead_at: m.dead_at,
      extra_info: m.extra_info,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      parent_id: m.parent_id,
      logo: m.logo,
    }
  }
}
