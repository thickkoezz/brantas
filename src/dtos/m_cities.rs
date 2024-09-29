use crate::entities::m_cities::{ActiveModel, Model};
use salvo::macros::Extractible;
use salvo::oapi::ToSchema;
use sea_orm::prelude::{DateTime, Decimal};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type ID = i32;

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct MCitiesDTO {
  pub id: i32,
  pub name: String,
  pub state_id: i16,
  pub state_code: String,
  pub country_id: i16,
  pub country_code: String,
  pub latitude: Decimal,
  pub longitude: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_at: Option<DateTime>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTime>,
  pub flag: i16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wikidataid: Option<String>,
}

impl MCitiesDTO {
  pub fn set_id(&mut self, v: i32) -> &mut Self {
    self.id = v;
    self
  }

  pub fn set_name(&mut self, v: String) -> &mut Self {
    self.name = v;
    self
  }

  pub fn set_state_id(&mut self, v: i16) -> &mut Self {
    self.state_id = v;
    self
  }

  pub fn set_state_code(&mut self, v: String) -> &mut Self {
    self.state_code = v;
    self
  }

  pub fn set_country_id(&mut self, v: i16) -> &mut Self {
    self.country_id = v;
    self
  }

  pub fn set_country_code(&mut self, v: String) -> &mut Self {
    self.country_code = v;
    self
  }

  pub fn set_latitude(&mut self, v: Decimal) -> &mut Self {
    self.latitude = v;
    self
  }

  pub fn set_longitude(&mut self, v: Decimal) -> &mut Self {
    self.longitude = v;
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

impl From<Model> for MCitiesDTO {
  fn from(m: Model) -> Self {
    Self {
      id: m.id,
      name: m.name,
      state_id: m.state_id,
      state_code: m.state_code,
      country_id: m.country_id,
      country_code: m.country_code,
      latitude: m.latitude,
      longitude: m.longitude,
      created_at: m.created_at,
      updated_at: m.updated_at,
      flag: m.flag,
      wikidataid: m.wikidataid,
    }
  }
}

impl From<ActiveModel> for MCitiesDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      id: m.id.unwrap(),
      name: m.name.unwrap(),
      state_id: m.state_id.unwrap(),
      state_code: m.state_code.unwrap(),
      country_id: m.country_id.unwrap(),
      country_code: m.country_code.unwrap(),
      latitude: m.latitude.unwrap(),
      longitude: m.longitude.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      flag: m.flag.unwrap(),
      wikidataid: m.wikidataid.unwrap(),
    }
  }
}
