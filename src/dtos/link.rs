use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct LinkAddRequest {
  pub id: Uuid,
  pub owner_id: Uuid,
  pub link_url: String,
  pub hashtag: Option<String>,
  pub use_count: i32,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct LinkUpdateRequest {
  pub id: Uuid,
  pub owner_id: Uuid,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub link_url: String,
  pub hashtag: Option<String>,
  pub use_count: i32,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct LinkResponse {
  pub id: Uuid,
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub link_url: String,
  pub hashtag: Option<String>,
  pub use_count: i32,
}
