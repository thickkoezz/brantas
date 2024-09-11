use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PhotoAddRequest {
  pub owner_id: Uuid,
  pub photo: String,
  pub size: i32,
  pub title: Option<String>,
  pub caption: Option<String>,
  pub code: Option<String>,
  pub slug: Option<String>,
}

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct PhotoUpdateRequest {
  pub owner_id: Uuid,
  pub photo: String,
  pub size: i32,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: Option<String>,
  pub caption: Option<String>,
  pub code: Option<String>,
  pub slug: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PhotoResponse {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub photo: String,
  pub size: i32,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: Option<String>,
  pub caption: Option<String>,
  pub code: Option<String>,
  pub slug: Option<String>,
}

impl From<crate::entities::photo> for PhotoResponse {
  fn from(m: crate::entities::photo) -> PhotoResponse {
    PhotoResponse {
      owner_id: m.owner_id,
      created_at: m.created_at,
      photo: m.photo,
      size: m.size,
      deleted_at: m.deleted_at,
      title: m.title,
      caption: m.caption,
      code: m.code,
      slug: m.slug,
    }
  }
}
