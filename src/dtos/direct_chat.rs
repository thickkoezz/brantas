use crate::entities::direct_chat::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct DirectChatDTO {
  pub sender_id: Uuid,
  pub receiver_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub content: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub replied_sender_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub replied_receiver_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub forwarded_sender_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub forwarded_receiver_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub forwarded_created_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub forwarded_group_creator_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub forwarded_group_created_at: Option<DateTimeWithTimeZone>,
  pub is_pinned: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pin_expired_at: Option<DateTimeWithTimeZone>,
}

impl DirectChatDTO {
  pub fn new(sender_id: Uuid, receiver_id: Uuid) -> Self {
    Self {
      sender_id,
      receiver_id,
      created_at: DateTimeWithTimeZone::from(Local::now()),
      ..Default::default()
    }
  }

  pub fn create(sender_id: Uuid, receiver_id: Uuid) -> Self {
    Self {
      ..Self::new(sender_id, receiver_id)
    }
  }

  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (
      self.sender_id.clone(),
      self.receiver_id.clone(),
      self.created_at.clone(),
    )
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.sender_id = v.0;
    self.receiver_id = v.1;
    self.created_at = v.2;
    self
  }

  pub fn set_sender_id(&mut self, v: Uuid) -> &mut Self {
    self.sender_id = v;
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

  pub fn set_content(&mut self, v: String) -> &mut Self {
    self.content = v;
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

  pub fn set_replied_sender_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.replied_sender_id = v;
    self
  }

  pub fn set_replied_receiver_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.replied_receiver_id = v;
    self
  }

  pub fn set_replied_created_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.replied_created_at = v;
    self
  }

  pub fn set_forwarded_sender_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.forwarded_sender_id = v;
    self
  }

  pub fn set_forwarded_receiver_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.forwarded_receiver_id = v;
    self
  }

  pub fn set_forwarded_created_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.forwarded_created_at = v;
    self
  }

  pub fn set_forwarded_group_creator_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.forwarded_group_creator_id = v;
    self
  }

  pub fn set_forwarded_group_created_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.forwarded_group_created_at = v;
    self
  }

  pub fn set_is_pinned(&mut self, v: bool) -> &mut Self {
    self.is_pinned = v;
    self
  }

  pub fn set_pin_expired_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.pin_expired_at = v;
    self
  }
}

impl From<Model> for DirectChatDTO {
  fn from(m: Model) -> Self {
    Self {
      sender_id: m.sender_id,
      receiver_id: m.receiver_id,
      created_at: m.created_at,
      content: m.content,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      replied_sender_id: m.replied_sender_id,
      replied_receiver_id: m.replied_receiver_id,
      replied_created_at: m.replied_created_at,
      forwarded_sender_id: m.forwarded_sender_id,
      forwarded_receiver_id: m.forwarded_receiver_id,
      forwarded_created_at: m.forwarded_created_at,
      forwarded_group_creator_id: m.forwarded_group_creator_id,
      forwarded_group_created_at: m.forwarded_group_created_at,
      is_pinned: m.is_pinned,
      pin_expired_at: m.pin_expired_at,
    }
  }
}

impl From<ActiveModel> for DirectChatDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      sender_id: m.sender_id.unwrap(),
      receiver_id: m.receiver_id.unwrap(),
      created_at: m.created_at.unwrap(),
      content: m.content.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      replied_sender_id: m.replied_sender_id.unwrap(),
      replied_receiver_id: m.replied_receiver_id.unwrap(),
      replied_created_at: m.replied_created_at.unwrap(),
      forwarded_sender_id: m.forwarded_sender_id.unwrap(),
      forwarded_receiver_id: m.forwarded_receiver_id.unwrap(),
      forwarded_created_at: m.forwarded_created_at.unwrap(),
      forwarded_group_creator_id: m.forwarded_group_creator_id.unwrap(),
      forwarded_group_created_at: m.forwarded_group_created_at.unwrap(),
      is_pinned: m.is_pinned.unwrap(),
      pin_expired_at: m.pin_expired_at.unwrap(),
    }
  }
}
