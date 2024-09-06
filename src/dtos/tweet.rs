use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::{DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct TweetAddRequest {
  pub owner_id: Uuid,
  pub tweet: String,
  pub is_published: bool,
  pub hashtag: Option<String>,
  pub replied_owner_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub retweet_owner_id: Option<Uuid>,
  pub retweet_created_at: Option<DateTimeWithTimeZone>,
  pub react_count: i32,
  pub reply_count: i32,
  pub retweet_count: i32,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct TweetUpdateRequest {
  pub owner_id: Uuid,
  pub tweet: String,
  pub is_published: bool,
  pub hashtag: Option<String>,
  pub replied_owner_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub retweet_owner_id: Option<Uuid>,
  pub retweet_created_at: Option<DateTimeWithTimeZone>,
  pub react_count: i32,
  pub reply_count: i32,
  pub retweet_count: i32,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct TweetResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub tweet: String,
  pub is_published: bool,
  pub hashtag: Option<String>,
  pub replied_owner_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub retweet_owner_id: Option<Uuid>,
  pub retweet_created_at: Option<DateTimeWithTimeZone>,
  pub react_count: i32,
  pub reply_count: i32,
  pub retweet_count: i32,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}