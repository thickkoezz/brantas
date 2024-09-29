use crate::entities::m_subregions::{ActiveModel, Model};
use salvo::macros::Extractible;
use salvo::oapi::ToSchema;
use sea_orm::prelude::{DateTime, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type ID = i16;

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct MSubregionsDTO {
  pub id: i16,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub translations: Option<Json>,
  pub region_id: i16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_at: Option<DateTime>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTime>,
  pub flag: i16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wikidataid: Option<String>,
}

impl MSubregionsDTO {
  pub fn set_id(&mut self, v: i16) -> &mut Self {
    self.id = v;
    self
  }

  pub fn set_name(&mut self, v: String) -> &mut Self {
    self.name = v;
    self
  }

  pub fn set_translations(&mut self, v: Option<Json>) -> &mut Self {
    self.translations = v;
    self
  }

  pub fn set_region_id(&mut self, v: i16) -> &mut Self {
    self.region_id = v;
    self
  }

  pub fn set_created_at(&mut self, v: Option<DateTime>) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_updated_at(&mut self, v: Option<DateTime>) -> &mut Self {
    self.updated_at = v;
    self
  }

  pub fn set_flag(&mut self, v: i16) -> &mut Self {
    self.flag = v;
    self
  }

  pub fn set_wikidataid(&mut self, v: Option<String>) -> &mut Self {
    self.wikidataid = v;
    self
  }
}

impl From<Model> for MSubregionsDTO {
  fn from(m: Model) -> Self {
    Self {
      id: m.id,
      name: m.name,
      translations: m.translations,
      region_id: m.region_id,
      created_at: m.created_at,
      updated_at: m.updated_at,
      flag: m.flag,
      wikidataid: m.wikidataid,
    }
  }
}

impl From<ActiveModel> for MSubregionsDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      id: m.id.unwrap(),
      name: m.name.unwrap(),
      translations: m.translations.unwrap(),
      region_id: m.region_id.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      flag: m.flag.unwrap(),
      wikidataid: m.wikidataid.unwrap(),
    }
  }
}
