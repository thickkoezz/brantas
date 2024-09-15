use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct OrganizationAddressAddRequest {
  pub organization_id: Uuid,
  pub city_id: i32,
  pub department_id: Uuid,
  pub description: Option<String>,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct OrganizationAddressUpdateRequest {
  pub organization_id: Uuid,
  pub city_id: i32,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct OrganizationAddressResponse {
  pub organization_id: Uuid,
  pub city_id: i32,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub description: Option<String>,
}

impl From<crate::entities::organization_address::Model> for OrganizationAddressResponse {
  fn from(m: crate::entities::organization_address::Model) -> Self {
    Self {
      organization_id: m.organization_id,
      city_id: m.city_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      description: m.description,
    }
  }
}

impl From<crate::entities::organization_address::ActiveModel> for OrganizationAddressResponse {
  fn from(m: crate::entities::organization_address::ActiveModel) -> Self {
    Self {
      organization_id: m.organization_id.unwrap(),
      city_id: m.city_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      description: m.description.unwrap(),
    }
  }
}
