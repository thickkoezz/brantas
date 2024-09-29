use crate::entities::group_chat_reaction::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid, Uuid, DateTimeWithTimeZone, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct GroupChatReactionDTO {
  pub owner_id: Uuid,
  pub reacted_group_chat_sender_id: Uuid,
  pub reacted_group_chat_group_creator_id: Uuid,
  pub reacted_group_chat_group_created_at: DateTimeWithTimeZone,
  pub reacted_group_chat_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  pub reaction_emoji: String,
}

impl GroupChatReactionDTO {
  pub fn get_id(&self) -> ID {
    (
      self.owner_id.clone(),
      self.reacted_group_chat_sender_id.clone(),
      self.reacted_group_chat_group_creator_id.clone(),
      self.reacted_group_chat_group_created_at.clone(),
      self.reacted_group_chat_created_at.clone(),
    )
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.owner_id = v.0;
    self.reacted_group_chat_sender_id = v.1;
    self.reacted_group_chat_group_creator_id = v.2;
    self.reacted_group_chat_group_created_at = v.3;
    self.reacted_group_chat_created_at = v.4;
    self
  }

  pub fn set_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.owner_id = v;
    self
  }

  pub fn set_reacted_group_chat_sender_id(&mut self, v: Uuid) -> &mut Self {
    self.reacted_group_chat_sender_id = v;
    self
  }

  pub fn set_reacted_group_chat_group_creator_id(&mut self, v: Uuid) -> &mut Self {
    self.reacted_group_chat_group_creator_id = v;
    self
  }

  pub fn set_reacted_group_chat_group_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.reacted_group_chat_group_created_at = v;
    self
  }

  pub fn set_reacted_group_chat_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.reacted_group_chat_created_at = v;
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

impl From<Model> for GroupChatReactionDTO {
  fn from(m: Model) -> Self {
    Self {
      owner_id: m.owner_id,
      reacted_group_chat_sender_id: m.reacted_group_chat_sender_id,
      reacted_group_chat_group_creator_id: m.reacted_group_chat_group_creator_id,
      reacted_group_chat_group_created_at: m.reacted_group_chat_group_created_at,
      reacted_group_chat_created_at: m.reacted_group_chat_created_at,
      created_at: m.created_at,
      reaction_emoji: m.reaction_emoji,
    }
  }
}

impl From<ActiveModel> for GroupChatReactionDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      reacted_group_chat_sender_id: m.reacted_group_chat_sender_id.unwrap(),
      reacted_group_chat_group_creator_id: m.reacted_group_chat_group_creator_id.unwrap(),
      reacted_group_chat_group_created_at: m.reacted_group_chat_group_created_at.unwrap(),
      reacted_group_chat_created_at: m.reacted_group_chat_created_at.unwrap(),
      created_at: m.created_at.unwrap(),
      reaction_emoji: m.reaction_emoji.unwrap(),
    }
  }
}
