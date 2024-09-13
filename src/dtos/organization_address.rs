use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct OrganizationAddressAddRequest {
  pub organization_id: Uuid,
  pub city_id: i32,
  pub department_id: Uuid,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct OrganizationAddressUpdateRequest {
  pub organization_id: Uuid,
  pub city_id: i32,
  pub department_id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct OrganizationAddressResponse {
  pub organization_id: Uuid,
  pub city_id: i32,
  pub department_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::organization_address::Model> for OrganizationAddressResponse {
  fn from(m: crate::entities::organization_address::Model) -> Self {
    Self {
      organization_id: m.organization_id,
      city_id: m.city_id,
      department_id: m.department_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
    }
  }
}
