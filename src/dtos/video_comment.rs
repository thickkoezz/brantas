use crate::entities::video_comment::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct VideoCommentDTO {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub commented_video_owner_id: Uuid,
  pub commented_video_created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub content: String,
  pub reaction_count: i32,
}

impl VideoCommentDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (self.owner_id.clone(), self.created_at.clone())
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.owner_id = v.0;
    self.created_at = v.1;
    self
  }

  pub fn set_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.owner_id = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_commented_video_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.commented_video_owner_id = v;
    self
  }

  pub fn set_commented_video_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.commented_video_created_at = v;
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

  pub fn set_content(&mut self, v: String) -> &mut Self {
    self.content = v;
    self
  }

  pub fn set_reaction_count(&mut self, v: i32) -> &mut Self {
    self.reaction_count = v;
    self
  }
}

impl From<Model> for VideoCommentDTO {
  fn from(m: Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      commented_video_owner_id: m.commented_video_owner_id,
      commented_video_created_at: m.commented_video_created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      content: m.content,
      reaction_count: m.reaction_count,
    }
  }
}

impl From<ActiveModel> for VideoCommentDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      commented_video_owner_id: m.commented_video_owner_id.unwrap(),
      commented_video_created_at: m.commented_video_created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      content: m.content.unwrap(),
      reaction_count: m.reaction_count.unwrap(),
    }
  }
}
