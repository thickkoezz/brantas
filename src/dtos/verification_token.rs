use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct VerificationTokenAddRequest {
  pub identifier: String,
  pub token: String,
  pub expires: DateTimeWithTimeZone,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct VerificationTokenResponse {
  pub identifier: String,
  pub token: String,
  pub expires: DateTimeWithTimeZone,
}

impl From<crate::entities::verification_token::Model> for VerificationTokenResponse {
  fn from(m: crate::entities::verification_token::Model) -> Self {
    Self {
      identifier: m.identifier,
      token: m.token,
      expires: m.expires,
    }
  }
}
