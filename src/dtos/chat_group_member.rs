use crate::entities::chat_group_member::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone, Uuid);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct ChatGroupMemberDTO {
  pub group_creator_id: Uuid,
  pub group_created_at: DateTimeWithTimeZone,
  pub member_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl ChatGroupMemberDTO {
  pub fn new() -> Self {
    Self {
      created_at: DateTimeWithTimeZone::from(Local::now()),
      ..Self::default()
    }
  }

  pub fn create(
    group_creator_id: Uuid,
    group_created_at: DateTimeWithTimeZone,
    member_id: Uuid,
  ) -> Self {
    Self {
      group_creator_id,
      group_created_at,
      member_id,
      created_at: DateTimeWithTimeZone::from(Local::now()),
      ..Default::default()
    }
  }

  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (
      self.group_creator_id.clone(),
      self.group_created_at.clone(),
      self.member_id.clone(),
    )
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.group_creator_id = v.0;
    self.group_created_at = v.1;
    self.member_id = v.2;
    self
  }

  pub fn set_group_creator_id(&mut self, v: Uuid) -> &mut Self {
    self.group_creator_id = v;
    self
  }

  pub fn set_group_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.group_created_at = v;
    self
  }

  pub fn set_member_id(&mut self, v: Uuid) -> &mut Self {
    self.member_id = v;
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

impl From<Model> for ChatGroupMemberDTO {
  fn from(m: Model) -> Self {
    Self {
      group_creator_id: m.group_creator_id,
      group_created_at: m.group_created_at,
      member_id: m.member_id,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<ActiveModel> for ChatGroupMemberDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      group_creator_id: m.group_creator_id.unwrap(),
      group_created_at: m.group_created_at.unwrap(),
      member_id: m.member_id.unwrap(),
      created_at: m.created_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
