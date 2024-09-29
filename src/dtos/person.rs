use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{Date, DateTimeWithTimeZone, Json};
use sea_orm::sqlx::types::chrono;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = Uuid;

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct PersonDTO {
  pub id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub first_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub middle_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dob: Option<Date>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sex: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deceased_at: Option<Date>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub extra_info: Option<Json>,
  pub is_deceased: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub photo: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  pub is_email_verified: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nickname: Option<String>,
}

impl PersonDTO {
  pub fn new() -> Self {
    Self {
      id: Uuid::new_v4(),
      created_at: DateTimeWithTimeZone::from(Local::now()),
      ..Default::default()
    }
  }

  pub fn create() -> Self {
    Self { ..Self::new() }
  }

  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(chrono::Local::now()));
    self
  }

  pub fn set_id(&mut self, v: Uuid) -> &mut Self {
    self.id = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_updated_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.updated_at = v;
    self
  }

  pub fn set_deleted_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.deleted_at = v;
    self
  }

  pub fn set_first_name(&mut self, v: String) -> &mut Self {
    self.first_name = v;
    self
  }

  pub fn set_middle_name(&mut self, v: Option<String>) -> &mut Self {
    self.middle_name = v;
    self
  }

  pub fn set_last_name(&mut self, v: Option<String>) -> &mut Self {
    self.last_name = v;
    self
  }

  pub fn set_dob(&mut self, v: Option<Date>) -> &mut Self {
    self.dob = v;
    self
  }

  pub fn set_sex(&mut self, v: Option<bool>) -> &mut Self {
    self.sex = v;
    self
  }

  pub fn set_deceased_at(&mut self, v: Option<Date>) -> &mut Self {
    self.deceased_at = v;
    self
  }

  pub fn set_extra_info(&mut self, v: Option<Json>) -> &mut Self {
    self.extra_info = v;
    self
  }

  pub fn set_is_deceased(&mut self, v: bool) -> &mut Self {
    self.is_deceased = v;
    self
  }

  pub fn set_photo(&mut self, v: Option<String>) -> &mut Self {
    self.photo = v;
    self
  }

  pub fn set_email(&mut self, v: Option<String>) -> &mut Self {
    self.email = v;
    self
  }

  pub fn set_is_email_verified(&mut self, v: bool) -> &mut Self {
    self.is_email_verified = v;
    self
  }

  pub fn set_nickname(&mut self, v: Option<String>) -> &mut Self {
    self.nickname = v;
    self
  }
}

impl From<crate::entities::person::Model> for PersonDTO {
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

impl From<crate::entities::person::ActiveModel> for PersonDTO {
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
