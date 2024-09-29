use crate::entities::starred_chat::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct StarredChatDTO {
  pub creator_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub chat_sender_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub direct_chat_receiver_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub group_chat_group_creator_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub group_chat_group_created_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub chat_created_at: Option<DateTimeWithTimeZone>,
}

impl StarredChatDTO {
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

  pub fn set_chat_sender_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.chat_sender_id = v;
    self
  }

  pub fn set_direct_chat_receiver_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.direct_chat_receiver_id = v;
    self
  }

  pub fn set_group_chat_group_creator_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.group_chat_group_creator_id = v;
    self
  }

  pub fn set_group_chat_group_created_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.group_chat_group_created_at = v;
    self
  }

  pub fn set_chat_created_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.chat_created_at = v;
    self
  }
}

impl From<Model> for StarredChatDTO {
  fn from(m: Model) -> Self {
    Self {
      creator_id: m.creator_id,
      created_at: m.created_at,
      chat_sender_id: m.chat_sender_id,
      direct_chat_receiver_id: m.direct_chat_receiver_id,
      group_chat_group_creator_id: m.group_chat_group_creator_id,
      group_chat_group_created_at: m.group_chat_group_created_at,
      chat_created_at: m.chat_created_at,
    }
  }
}

impl From<ActiveModel> for StarredChatDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      creator_id: m.creator_id.unwrap(),
      created_at: m.created_at.unwrap(),
      chat_sender_id: m.chat_sender_id.unwrap(),
      direct_chat_receiver_id: m.direct_chat_receiver_id.unwrap(),
      group_chat_group_creator_id: m.group_chat_group_creator_id.unwrap(),
      group_chat_group_created_at: m.group_chat_group_created_at.unwrap(),
      chat_created_at: m.chat_created_at.unwrap(),
    }
  }
}
