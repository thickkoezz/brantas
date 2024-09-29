use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct PostDTO {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: String,
  pub content: String,
  pub is_published: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub hashtag: Option<String>,
  pub view_count: i32,
  pub comment_count: i32,
  pub reaction_count: i32,
  pub is_public: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub group_name: Option<String>,
  pub can_comment: bool,
}

impl PostDTO {
  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(chrono::Local::now()));
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

  pub fn set_updated_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.updated_at = v;
    self
  }

  pub fn set_deleted_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.deleted_at = v;
    self
  }

  pub fn set_title(&mut self, v: String) -> &mut Self {
    self.title = v;
    self
  }

  pub fn set_content(&mut self, v: String) -> &mut Self {
    self.content = v;
    self
  }

  pub fn set_is_published(&mut self, v: bool) -> &mut Self {
    self.is_published = v;
    self
  }

  pub fn set_hashtag(&mut self, v: Option<String>) -> &mut Self {
    self.hashtag = v;
    self
  }

  pub fn set_view_count(&mut self, v: i32) -> &mut Self {
    self.view_count = v;
    self
  }

  pub fn set_comment_count(&mut self, v: i32) -> &mut Self {
    self.comment_count = v;
    self
  }

  pub fn set_reaction_count(&mut self, v: i32) -> &mut Self {
    self.reaction_count = v;
    self
  }

  pub fn set_is_public(&mut self, v: bool) -> &mut Self {
    self.is_public = v;
    self
  }

  pub fn set_group_name(&mut self, v: Option<String>) -> &mut Self {
    self.group_name = v;
    self
  }

  pub fn set_can_comment(&mut self, v: bool) -> &mut Self {
    self.can_comment = v;
    self
  }
}

impl From<crate::entities::post::Model> for PostDTO {
  fn from(m: crate::entities::post::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      title: m.title,
      content: m.content,
      is_published: m.is_published,
      hashtag: m.hashtag,
      view_count: m.view_count,
      comment_count: m.comment_count,
      reaction_count: m.reaction_count,
      is_public: m.is_public,
      group_name: m.group_name,
      can_comment: m.can_comment,
    }
  }
}

impl From<crate::entities::post::ActiveModel> for PostDTO {
  fn from(m: crate::entities::post::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      title: m.title.unwrap(),
      content: m.content.unwrap(),
      is_published: m.is_published.unwrap(),
      hashtag: m.hashtag.unwrap(),
      view_count: m.view_count.unwrap(),
      comment_count: m.comment_count.unwrap(),
      reaction_count: m.reaction_count.unwrap(),
      is_public: m.is_public.unwrap(),
      group_name: m.group_name.unwrap(),
      can_comment: m.can_comment.unwrap(),
    }
  }
}
