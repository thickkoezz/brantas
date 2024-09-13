use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct OrganizationAdministratorAddRequest {
  pub organization_id: Uuid,
  pub administrator_id: Uuid,
  pub department_id: Uuid,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct OrganizationAdministratorResponse {
  pub organization_id: Uuid,
  pub administrator_id: Uuid,
  pub department_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<crate::entities::organization_administrator::Model> for OrganizationAdministratorResponse {
  fn from(m: crate::entities::organization_administrator::Model) -> Self {
    Self {
      organization_id: m.organization_id,
      administrator_id: m.administrator_id,
      department_id: m.department_id,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
    }
  }
}
