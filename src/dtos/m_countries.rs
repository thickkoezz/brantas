use crate::entities::m_countries::{ActiveModel, Model};
use salvo::macros::Extractible;
use salvo::oapi::ToSchema;
use sea_orm::prelude::{DateTime, DateTimeWithTimeZone, Decimal, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type ID = i16;

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct MCountriesDTO {
  pub id: i16,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iso3: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub numeric_code: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iso2: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phonecode: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub capital: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub currency: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub currency_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub currency_symbol: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tld: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub native: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub region: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub region_id: Option<i16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subregion: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subregion_id: Option<i16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nationality: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub timezones: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub translations: Option<Json>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub latitude: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub longitude: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub emoji: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub emojiu: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_at: Option<DateTime>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTime>,
  pub flag: i16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wikidataid: Option<String>,
}

impl MCountriesDTO {
  pub fn set_id(&mut self, v: i16) -> &mut Self {
    self.id = v;
    self
  }

  pub fn set_name(&mut self, v: String) -> &mut Self {
    self.name = v;
    self
  }

  pub fn set_iso3(&mut self, v: Option<String>) -> &mut Self {
    self.iso3 = v;
    self
  }

  pub fn set_numeric_code(&mut self, v: Option<String>) -> &mut Self {
    self.numeric_code = v;
    self
  }

  pub fn set_iso2(&mut self, v: Option<String>) -> &mut Self {
    self.iso2 = v;
    self
  }

  pub fn set_phonecode(&mut self, v: Option<String>) -> &mut Self {
    self.phonecode = v;
    self
  }

  pub fn set_capital(&mut self, v: Option<String>) -> &mut Self {
    self.capital = v;
    self
  }

  pub fn set_currency(&mut self, v: Option<String>) -> &mut Self {
    self.currency = v;
    self
  }

  pub fn set_currency_name(&mut self, v: Option<String>) -> &mut Self {
    self.currency_name = v;
    self
  }

  pub fn set_currency_symbol(&mut self, v: Option<String>) -> &mut Self {
    self.currency_symbol = v;
    self
  }

  pub fn set_tld(&mut self, v: Option<String>) -> &mut Self {
    self.tld = v;
    self
  }

  pub fn set_native(&mut self, v: Option<String>) -> &mut Self {
    self.native = v;
    self
  }

  pub fn set_region(&mut self, v: Option<String>) -> &mut Self {
    self.region = v;
    self
  }

  pub fn set_region_id(&mut self, v: Option<i16>) -> &mut Self {
    self.region_id = v;
    self
  }

  pub fn set_subregion(&mut self, v: Option<String>) -> &mut Self {
    self.subregion = v;
    self
  }

  pub fn set_subregion_id(&mut self, v: Option<i16>) -> &mut Self {
    self.subregion_id = v;
    self
  }

  pub fn set_nationality(&mut self, v: Option<String>) -> &mut Self {
    self.nationality = v;
    self
  }

  pub fn set_timezones(&mut self, v: Option<String>) -> &mut Self {
    self.timezones = v;
    self
  }

  pub fn set_translations(&mut self, v: Option<Json>) -> &mut Self {
    self.translations = v;
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

  pub fn set_emoji(&mut self, v: Option<String>) -> &mut Self {
    self.emoji = v;
    self
  }

  pub fn set_emojiu(&mut self, v: Option<String>) -> &mut Self {
    self.emojiu = v;
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

impl From<Model> for MCountriesDTO {
  fn from(m: Model) -> Self {
    Self {
      id: m.id,
      name: m.name,
      iso3: m.iso3,
      numeric_code: m.numeric_code,
      iso2: m.iso2,
      phonecode: m.phonecode,
      capital: m.capital,
      currency: m.currency,
      currency_name: m.currency_name,
      currency_symbol: m.currency_symbol,
      tld: m.tld,
      native: m.native,
      region: m.region,
      region_id: m.region_id,
      subregion: m.subregion,
      subregion_id: m.subregion_id,
      nationality: m.nationality,
      timezones: m.timezones,
      translations: m.translations,
      latitude: m.latitude,
      longitude: m.longitude,
      emoji: m.emoji,
      emojiu: m.emojiu,
      created_at: m.created_at,
      updated_at: m.updated_at,
      flag: m.flag,
      wikidataid: m.wikidataid,
    }
  }
}

impl From<ActiveModel> for MCountriesDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      id: m.id.unwrap(),
      name: m.name.unwrap(),
      iso3: m.iso3.unwrap(),
      numeric_code: m.numeric_code.unwrap(),
      iso2: m.iso2.unwrap(),
      phonecode: m.phonecode.unwrap(),
      capital: m.capital.unwrap(),
      currency: m.currency.unwrap(),
      currency_name: m.currency_name.unwrap(),
      currency_symbol: m.currency_symbol.unwrap(),
      tld: m.tld.unwrap(),
      native: m.native.unwrap(),
      region: m.region.unwrap(),
      region_id: m.region_id.unwrap(),
      subregion: m.subregion.unwrap(),
      subregion_id: m.subregion_id.unwrap(),
      nationality: m.nationality.unwrap(),
      timezones: m.timezones.unwrap(),
      translations: m.translations.unwrap(),
      latitude: m.latitude.unwrap(),
      longitude: m.longitude.unwrap(),
      emoji: m.emoji.unwrap(),
      emojiu: m.emojiu.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      flag: m.flag.unwrap(),
      wikidataid: m.wikidataid.unwrap(),
    }
  }
}
