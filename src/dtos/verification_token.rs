use crate::entities::verification_token::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type ID = (String, String);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct VerificationTokenDTO {
  pub identifier: String,
  pub token: String,
  pub expires: DateTimeWithTimeZone,
}

impl VerificationTokenDTO {
  pub fn get_id(&self) -> ID {
    (self.identifier.clone(), self.token.clone())
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.identifier = v.0;
    self.token = v.1;
    self
  }

  pub fn set_identifier(&mut self, v: String) -> &mut Self {
    self.identifier = v;
    self
  }

  pub fn set_token(&mut self, v: String) -> &mut Self {
    self.token = v;
    self
  }

  pub fn set_expires(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.expires = v;
    self
  }
}

impl From<Model> for VerificationTokenDTO {
  fn from(m: Model) -> Self {
    Self {
      identifier: m.identifier,
      token: m.token,
      expires: m.expires,
    }
  }
}

impl From<ActiveModel> for VerificationTokenDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      identifier: m.identifier.unwrap(),
      token: m.token.unwrap(),
      expires: m.expires.unwrap(),
    }
  }
}
