use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct PostReactionDTO {
  pub owner_id: Uuid,
  pub reacted_post_owner_id: Uuid,
  pub reacted_post_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  pub reaction_emoji: String,
}

impl PostReactionDTO {
  pub fn set_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.owner_id = v;
    self
  }

  pub fn set_reacted_post_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.reacted_post_owner_id = v;
    self
  }

  pub fn set_reacted_post_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.reacted_post_created_at = v;
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

impl From<crate::entities::post_reaction::Model> for PostReactionDTO {
  fn from(m: crate::entities::post_reaction::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      reacted_post_owner_id: m.reacted_post_owner_id,
      reacted_post_created_at: m.reacted_post_created_at,
      created_at: m.created_at,
      reaction_emoji: m.reaction_emoji,
    }
  }
}

impl From<crate::entities::post_reaction::ActiveModel> for PostReactionDTO {
  fn from(m: crate::entities::post_reaction::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      reacted_post_owner_id: m.reacted_post_owner_id.unwrap(),
      reacted_post_created_at: m.reacted_post_created_at.unwrap(),
      created_at: m.created_at.unwrap(),
      reaction_emoji: m.reaction_emoji.unwrap(),
    }
  }
}
