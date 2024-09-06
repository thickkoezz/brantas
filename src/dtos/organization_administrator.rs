use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone};
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