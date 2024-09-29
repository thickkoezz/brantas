use crate::entities::direct_chat_reaction::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid, Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct DirectChatReactionDTO {
  pub owner_id: Uuid,
  pub reacted_direct_chat_sender_id: Uuid,
  pub reacted_direct_chat_receiver_id: Uuid,
  pub reacted_direct_chat_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  pub reaction_emoji: String,
}

impl DirectChatReactionDTO {
  pub fn new(
    owner_id: Uuid,
    reacted_direct_chat_sender_id: Uuid,
    reacted_direct_chat_receiver_id: Uuid,
    reacted_direct_chat_created_at: DateTimeWithTimeZone,
  ) -> Self {
    Self {
      owner_id,
      reacted_direct_chat_sender_id,
      reacted_direct_chat_receiver_id,
      reacted_direct_chat_created_at,
      created_at: DateTimeWithTimeZone::from(Local::now()),
      ..Default::default()
    }
  }

  pub fn create(
    owner_id: Uuid,
    reacted_direct_chat_sender_id: Uuid,
    reacted_direct_chat_receiver_id: Uuid,
    reacted_direct_chat_created_at: DateTimeWithTimeZone,
    reaction_emoji: String,
  ) -> Self {
    Self {
      reaction_emoji,
      ..Self::new(
        owner_id,
        reacted_direct_chat_sender_id,
        reacted_direct_chat_receiver_id,
        reacted_direct_chat_created_at,
      )
    }
  }

  pub fn get_id(&self) -> ID {
    (
      self.owner_id.clone(),
      self.reacted_direct_chat_sender_id.clone(),
      self.reacted_direct_chat_receiver_id.clone(),
      self.reacted_direct_chat_created_at,
    )
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.owner_id = v.0;
    self.reacted_direct_chat_sender_id = v.1;
    self.reacted_direct_chat_receiver_id = v.2;
    self.reacted_direct_chat_created_at = v.3;
    self
  }

  pub fn set_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.owner_id = v;
    self
  }

  pub fn set_reacted_direct_chat_sender_id(&mut self, v: Uuid) -> &mut Self {
    self.reacted_direct_chat_sender_id = v;
    self
  }

  pub fn set_reacted_direct_chat_receiver_id(&mut self, v: Uuid) -> &mut Self {
    self.reacted_direct_chat_receiver_id = v;
    self
  }

  pub fn set_reacted_direct_chat_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.reacted_direct_chat_created_at = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_reaction_emoji(&mut self, v: String) -> &mut Self {
    self.reaction_emoji = v;
    self
  }
}

impl From<Model> for DirectChatReactionDTO {
  fn from(m: Model) -> Self {
    Self {
      owner_id: m.owner_id,
      reacted_direct_chat_sender_id: m.reacted_direct_chat_sender_id,
      reacted_direct_chat_receiver_id: m.reacted_direct_chat_receiver_id,
      reacted_direct_chat_created_at: m.reacted_direct_chat_created_at,
      created_at: m.created_at,
      reaction_emoji: m.reaction_emoji,
    }
  }
}

impl From<ActiveModel> for DirectChatReactionDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      reacted_direct_chat_sender_id: m.reacted_direct_chat_sender_id.unwrap(),
      reacted_direct_chat_receiver_id: m.reacted_direct_chat_receiver_id.unwrap(),
      reacted_direct_chat_created_at: m.reacted_direct_chat_created_at.unwrap(),
      created_at: m.created_at.unwrap(),
      reaction_emoji: m.reaction_emoji.unwrap(),
    }
  }
}
