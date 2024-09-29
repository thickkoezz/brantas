use crate::entities::m_regions::{ActiveModel, Model};
use salvo::macros::Extractible;
use salvo::oapi::ToSchema;
use sea_orm::prelude::{DateTime, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type ID = i16;

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct MRegionsDTO {
  pub id: i16,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub translations: Option<Json>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_at: Option<DateTime>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTime>,
  pub flag: i16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wikidataid: Option<String>,
}

impl From<Model> for MRegionsDTO {
  fn from(m: Model) -> Self {
    Self {
      id: m.id,
      name: m.name,
      translations: m.translations,
      created_at: m.created_at,
      updated_at: m.updated_at,
      flag: m.flag,
      wikidataid: m.wikidataid,
    }
  }
}

impl From<ActiveModel> for MRegionsDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      id: m.id.unwrap(),
      name: m.name.unwrap(),
      translations: m.translations.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      flag: m.flag.unwrap(),
      wikidataid: m.wikidataid.unwrap(),
    }
  }
}
