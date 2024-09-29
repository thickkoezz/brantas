use crate::entities::m_states::{ActiveModel, Model};
use salvo::macros::Extractible;
use salvo::oapi::ToSchema;
use sea_orm::prelude::{DateTime, Decimal};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type ID = i16;

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct MStatesDTO {
  pub id: i16,
  pub name: String,
  pub country_id: i16,
  pub country_code: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub fips_code: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iso2: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub latitude: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub longitude: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_at: Option<DateTime>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTime>,
  pub flag: i16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wikidataid: Option<String>,
}

impl MStatesDTO {
  pub fn set_id(&mut self, v: i16) -> &mut Self {
    self.id = v;
    self
  }

  pub fn set_name(&mut self, name: &str) -> &mut Self {
    self.name = name.to_string();
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

  pub fn set_fips_code(&mut self, v: Option<String>) -> &mut Self {
    self.fips_code = v;
    self
  }

  pub fn set_iso2(&mut self, v: Option<String>) -> &mut Self {
    self.iso2 = v;
    self
  }

  pub fn set_type(&mut self, v: Option<String>) -> &mut Self {
    self.r#type = v;
    self
  }

  pub fn set_latitude(&mut self, v: Option<Decimal>) -> &mut Self {
    self.latitude = v;
    self
  }

  pub fn set_longitude(&mut self, v: Option<Decimal>) -> &mut Self {
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

impl From<Model> for MStatesDTO {
  fn from(m: Model) -> Self {
    Self {
      id: m.id,
      name: m.name,
      country_id: m.country_id,
      country_code: m.country_code,
      fips_code: m.fips_code,
      iso2: m.iso2,
      r#type: m.r#type,
      latitude: m.latitude,
      longitude: m.longitude,
      created_at: m.created_at,
      updated_at: m.updated_at,
      flag: m.flag,
      wikidataid: m.wikidataid,
    }
  }
}

impl From<ActiveModel> for MStatesDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      id: m.id.unwrap(),
      name: m.name.unwrap(),
      country_id: m.country_id.unwrap(),
      country_code: m.country_code.unwrap(),
      fips_code: m.fips_code.unwrap(),
      iso2: m.iso2.unwrap(),
      r#type: m.r#type.unwrap(),
      latitude: m.latitude.unwrap(),
      longitude: m.longitude.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      flag: m.flag.unwrap(),
      wikidataid: m.wikidataid.unwrap(),
    }
  }
}
