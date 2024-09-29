use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct TweetDTO {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub tweet: String,
  pub is_published: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub hashtag: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub replied_owner_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub retweeted_owner_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub retweeted_created_at: Option<DateTimeWithTimeZone>,
  pub reaction_count: i32,
  pub reply_count: i32,
  pub retweet_count: i32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl TweetDTO {
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

  pub fn set_tweet(&mut self, v: String) -> &mut Self {
    self.tweet = v;
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

  pub fn set_replied_owner_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.replied_owner_id = v;
    self
  }

  pub fn set_replied_created_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.replied_created_at = v;
    self
  }

  pub fn set_retweeted_owner_id(&mut self, v: Option<Uuid>) -> &mut Self {
    self.retweeted_owner_id = v;
    self
  }

  pub fn set_retweeted_created_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.retweeted_created_at = v;
    self
  }

  pub fn set_reaction_count(&mut self, v: i32) -> &mut Self {
    self.reaction_count = v;
    self
  }

  pub fn set_reply_count(&mut self, v: i32) -> &mut Self {
    self.reply_count = v;
    self
  }

  pub fn set_retweet_count(&mut self, v: i32) -> &mut Self {
    self.retweet_count = v;
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
}

impl From<crate::entities::tweet::Model> for TweetDTO {
  fn from(m: crate::entities::tweet::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      tweet: m.tweet,
      is_published: m.is_published,
      hashtag: m.hashtag,
      replied_owner_id: m.replied_owner_id,
      replied_created_at: m.replied_created_at,
      retweeted_owner_id: m.retweeted_owner_id,
      retweeted_created_at: m.retweeted_created_at,
      reaction_count: m.reaction_count,
      reply_count: m.reply_count,
      retweet_count: m.retweet_count,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<crate::entities::tweet::ActiveModel> for TweetDTO {
  fn from(m: crate::entities::tweet::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      tweet: m.tweet.unwrap(),
      is_published: m.is_published.unwrap(),
      hashtag: m.hashtag.unwrap(),
      replied_owner_id: m.replied_owner_id.unwrap(),
      replied_created_at: m.replied_created_at.unwrap(),
      retweeted_owner_id: m.retweeted_owner_id.unwrap(),
      retweeted_created_at: m.retweeted_created_at.unwrap(),
      reaction_count: m.reaction_count.unwrap(),
      reply_count: m.reply_count.unwrap(),
      retweet_count: m.retweet_count.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
