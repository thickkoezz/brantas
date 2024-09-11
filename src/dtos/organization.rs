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

impl From<crate::entities::organization> for OrganizationResponse {
  fn from(m: crate::entities::organization) -> OrganizationResponse {
    OrganizationResponse {
      id: m.id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      name: m.name,
      abbreviation: m.abbreviation,
      description: m.description,
      dob: m.dob,
      dead_at: m.dead_at,
      extra_info: m.extra_info,
      is_dead: m.is_dead,
      parent_id: m.parent_id,
      logo: m.logo,
    }
  }
}
