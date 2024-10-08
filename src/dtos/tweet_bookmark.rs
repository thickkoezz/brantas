use crate::entities::tweet_bookmark::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, Uuid, DateTimeWithTimeZone);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct TweetBookmarkDTO {
  pub owner_id: Uuid,
  pub bookmarked_tweet_owner_id: Uuid,
  pub bookmarked_tweet_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
}

impl TweetBookmarkDTO {
  pub fn get_id(&self) -> ID {
    (
      self.owner_id.clone(),
      self.bookmarked_tweet_owner_id.clone(),
      self.bookmarked_tweet_created_at.clone(),
    )
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.owner_id = v.0;
    self.bookmarked_tweet_owner_id = v.1;
    self.bookmarked_tweet_created_at = v.2;
    self
  }

  pub fn set_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.owner_id = v;
    self
  }

  pub fn set_bookmarked_tweet_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.bookmarked_tweet_owner_id = v;
    self
  }

  pub fn set_bookmarked_tweet_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.bookmarked_tweet_created_at = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }
}

impl From<Model> for TweetBookmarkDTO {
  fn from(m: Model) -> Self {
    Self {
      owner_id: m.owner_id,
      bookmarked_tweet_owner_id: m.bookmarked_tweet_owner_id,
      bookmarked_tweet_created_at: m.bookmarked_tweet_created_at,
      created_at: m.created_at,
    }
  }
}

impl From<ActiveModel> for TweetBookmarkDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      bookmarked_tweet_owner_id: m.bookmarked_tweet_owner_id.unwrap(),
      bookmarked_tweet_created_at: m.bookmarked_tweet_created_at.unwrap(),
      created_at: m.created_at.unwrap(),
    }
  }
}
