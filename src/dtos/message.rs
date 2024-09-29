use crate::entities::message::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct MessageDTO {
  pub owner_id: Uuid,
  pub receiver_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub message: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl MessageDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (
      self.owner_id.clone(),
      self.receiver_id.clone(),
      self.created_at.clone(),
    )
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.owner_id = v.0;
    self.receiver_id = v.1;
    self.created_at = v.2;
    self
  }

  pub fn set_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.owner_id = v;
    self
  }

  pub fn set_receiver_id(&mut self, v: Uuid) -> &mut Self {
    self.receiver_id = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_message(&mut self, v: String) -> &mut Self {
    self.message = v;
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
}

impl From<Model> for MessageDTO {
  fn from(m: Model) -> Self {
    Self {
      owner_id: m.owner_id,
      receiver_id: m.receiver_id,
      created_at: m.created_at,
      message: m.message,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<ActiveModel> for MessageDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      receiver_id: m.receiver_id.unwrap(),
      created_at: m.created_at.unwrap(),
      message: m.message.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
