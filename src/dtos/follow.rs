use crate::entities::follow::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct FollowDTO {
  pub follower_id: Uuid,
  pub target_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl FollowDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (self.follower_id.clone(), self.target_id.clone())
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.follower_id = v.0;
    self.target_id = v.1;
    self
  }

  pub fn set_follower_id(&mut self, v: Uuid) -> &mut Self {
    self.follower_id = v;
    self
  }

  pub fn set_target_id(&mut self, v: Uuid) -> &mut Self {
    self.target_id = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_deleted_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.deleted_at = v;
    self
  }
}

impl From<Model> for FollowDTO {
  fn from(m: Model) -> Self {
    Self {
      follower_id: m.follower_id,
      target_id: m.target_id,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<ActiveModel> for FollowDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      follower_id: m.follower_id.unwrap(),
      target_id: m.target_id.unwrap(),
      created_at: m.created_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
