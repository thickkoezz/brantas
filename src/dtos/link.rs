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

impl From<crate::entities::link::Model> for LinkResponse {
  fn from(m: crate::entities::link::Model) -> Self {
    Self {
      id: m.id,
      owner_id: m.owner_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      link_url: m.link_url,
      hashtag: m.hashtag,
      use_count: m.use_count,
    }
  }
}
