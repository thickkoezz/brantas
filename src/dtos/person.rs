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
  pub nickname: Option<String>,
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
  pub nickname: Option<String>,
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
  pub nickname: Option<String>,
}

impl From<crate::entities::person::Model> for PersonResponse {
  fn from(m: crate::entities::person::Model) -> Self {
    Self {
      id: m.id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      first_name: m.first_name,
      middle_name: m.middle_name,
      last_name: m.last_name,
      dob: m.dob,
      sex: m.sex,
      deceased_at: m.deceased_at,
      extra_info: m.extra_info,
      is_deceased: m.is_deceased,
      photo: m.photo,
      email: m.email,
      is_email_verified: m.is_email_verified,
      nickname: m.nickname,
    }
  }
}

impl From<crate::entities::person::ActiveModel> for PersonResponse {
  fn from(m: crate::entities::person::ActiveModel) -> Self {
    Self {
      id: m.id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      first_name: m.first_name.unwrap(),
      middle_name: m.middle_name.unwrap(),
      last_name: m.last_name.unwrap(),
      dob: m.dob.unwrap(),
      sex: m.sex.unwrap(),
      deceased_at: m.deceased_at.unwrap(),
      extra_info: m.extra_info.unwrap(),
      is_deceased: m.is_deceased.unwrap(),
      photo: m.photo.unwrap(),
      email: m.email.unwrap(),
      is_email_verified: m.is_email_verified.unwrap(),
      nickname: m.nickname.unwrap(),
    }
  }
}
