use crate::entities::friend_group_target::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, String, Uuid);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct FriendGroupTargetDTO {
  pub group_owner_id: Uuid,
  pub group_name: String,
  pub target_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl FriendGroupTargetDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (
      self.group_owner_id.clone(),
      self.group_name.clone(),
      self.target_id.clone(),
    )
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.group_owner_id = v.0;
    self.group_name = v.1;
    self.target_id = v.2;
    self
  }

  pub fn set_group_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.group_owner_id = v;
    self
  }

  pub fn set_group_name(&mut self, v: String) -> &mut Self {
    self.group_name = v;
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

impl From<Model> for FriendGroupTargetDTO {
  fn from(m: Model) -> Self {
    Self {
      group_owner_id: m.group_owner_id,
      group_name: m.group_name,
      target_id: m.target_id,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<ActiveModel> for FriendGroupTargetDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      group_owner_id: m.group_owner_id.unwrap(),
      group_name: m.group_name.unwrap(),
      target_id: m.target_id.unwrap(),
      created_at: m.created_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
