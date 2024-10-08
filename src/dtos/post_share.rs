use crate::entities::post_share::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone, Uuid);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct PostShareDTO {
  pub post_owner_id: Uuid,
  pub post_created_at: DateTimeWithTimeZone,
  pub target_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub can_comment: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl PostShareDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (
      self.post_owner_id.clone(),
      self.post_created_at.clone(),
      self.target_id.clone(),
    )
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.post_owner_id = v.0;
    self.post_created_at = v.1;
    self.target_id = v.2;
    self
  }

  pub fn set_post_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.post_owner_id = v;
    self
  }

  pub fn set_post_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.post_created_at = v;
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

  pub fn set_can_comment(&mut self, v: bool) -> &mut Self {
    self.can_comment = v;
    self
  }

  pub fn set_deleted_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.deleted_at = v;
    self
  }
}

impl From<Model> for PostShareDTO {
  fn from(m: Model) -> Self {
    Self {
      post_owner_id: m.post_owner_id,
      post_created_at: m.post_created_at,
      target_id: m.target_id,
      created_at: m.created_at,
      can_comment: m.can_comment,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<ActiveModel> for PostShareDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      post_owner_id: m.post_owner_id.unwrap(),
      post_created_at: m.post_created_at.unwrap(),
      target_id: m.target_id.unwrap(),
      created_at: m.created_at.unwrap(),
      can_comment: m.can_comment.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
