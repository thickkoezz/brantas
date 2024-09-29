use crate::entities::chat_group::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct ChatGroupDTO {
  pub creator_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  pub is_public: bool,
  pub is_suspended: bool,
  pub is_channel: bool,
}

impl ChatGroupDTO {
  pub fn new(creator_id: Uuid) -> Self {
    Self {
      creator_id,
      created_at: DateTimeWithTimeZone::from(chrono::Local::now()),
      ..Default::default()
    }
  }

  pub fn create(
    creator_id: Uuid,
    name: Option<String>,
    is_public: bool,
    is_suspended: bool,
    is_channel: bool,
  ) -> Self {
    Self {
      creator_id,
      created_at: DateTimeWithTimeZone::from(Local::now()),
      name,
      is_public,
      is_suspended,
      is_channel,
      ..Self::new(creator_id)
    }
  }

  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (self.creator_id.clone(), self.created_at.clone())
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.creator_id = v.0;
    self.created_at = v.1;
    self
  }

  pub fn set_creator_id(&mut self, v: Uuid) -> &mut Self {
    self.creator_id = v;
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

  pub fn set_name(&mut self, v: Option<String>) -> &mut Self {
    self.name = v;
    self
  }

  pub fn set_is_public(&mut self, v: bool) -> &mut Self {
    self.is_public = v;
    self
  }

  pub fn set_is_suspended(&mut self, v: bool) -> &mut Self {
    self.is_suspended = v;
    self
  }

  pub fn set_is_channel(&mut self, v: bool) -> &mut Self {
    self.is_channel = v;
    self
  }
}

impl From<Model> for ChatGroupDTO {
  fn from(m: Model) -> Self {
    Self {
      creator_id: m.creator_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      name: m.name,
      is_public: m.is_public,
      is_suspended: m.is_suspended,
      is_channel: m.is_channel,
    }
  }
}

impl From<ActiveModel> for ChatGroupDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      creator_id: m.creator_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      name: m.name.unwrap(),
      is_public: m.is_public.unwrap(),
      is_suspended: m.is_suspended.unwrap(),
      is_channel: m.is_channel.unwrap(),
    }
  }
}
