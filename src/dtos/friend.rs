use crate::entities::friend::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct FriendDTO {
  pub invitor_id: Uuid,
  pub invitee_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}
impl FriendDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (self.invitor_id.clone(), self.invitee_id.clone())
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.invitor_id = v.0;
    self.invitee_id = v.1;
    self
  }

  pub fn set_invitor_id(&mut self, v: Uuid) -> &mut Self {
    self.invitor_id = v;
    self
  }

  pub fn set_invitee_id(&mut self, v: Uuid) -> &mut Self {
    self.invitee_id = v;
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

impl From<Model> for FriendDTO {
  fn from(m: Model) -> Self {
    Self {
      invitor_id: m.invitor_id,
      invitee_id: m.invitee_id,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<ActiveModel> for FriendDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      invitor_id: m.invitor_id.unwrap(),
      invitee_id: m.invitee_id.unwrap(),
      created_at: m.created_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
