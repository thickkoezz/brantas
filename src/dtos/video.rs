use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct VideoAddRequest {
  pub owner_id: Uuid,
  pub video: String,
  pub size: i32,
  pub title: Option<String>,
  pub caption: Option<String>,
  pub code: Option<String>,
  pub slug: Option<String>,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct VideoUpdateRequest {
  pub owner_id: Uuid,
  pub video: String,
  pub size: i32,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: Option<String>,
  pub caption: Option<String>,
  pub code: Option<String>,
  pub slug: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct VideoResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub video: String,
  pub size: i32,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: Option<String>,
  pub caption: Option<String>,
  pub code: Option<String>,
  pub slug: Option<String>,
}

impl From<crate::entities::video::Model> for VideoResponse {
  fn from(m: crate::entities::video::Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      video: m.video,
      size: m.size,
      deleted_at: m.deleted_at,
      title: m.title,
      caption: m.caption,
      code: m.code,
      slug: m.slug,
    }
  }
}

impl From<crate::entities::video::ActiveModel> for VideoResponse {
  fn from(m: crate::entities::video::ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      video: m.video.unwrap(),
      size: m.size.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      title: m.title.unwrap(),
      caption: m.caption.unwrap(),
      code: m.code.unwrap(),
      slug: m.slug.unwrap(),
    }
  }
}
