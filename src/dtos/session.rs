use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct SessionAddRequest {
  pub id: Uuid,
  pub session_token: String,
  pub user_account_id: Uuid,
  pub expires: DateTimeWithTimeZone,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct SessionResponse {
  pub id: Uuid,
  pub session_token: String,
  pub user_account_id: Uuid,
  pub expires: DateTimeWithTimeZone,
}

impl From<crate::entities::session::Model> for SessionResponse {
  fn from(m: crate::entities::session::Model) -> Self {
    Self {
      id: m.id,
      session_token: m.session_token,
      user_account_id: m.user_account_id,
      expires: m.expires,
    }
  }
}

impl From<crate::entities::session::ActiveModel> for SessionResponse {
  fn from(m: crate::entities::session::ActiveModel) -> Self {
    Self {
      id: m.id.unwrap(),
      session_token: m.session_token.unwrap(),
      user_account_id: m.user_account_id.unwrap(),
      expires: m.expires.unwrap(),
    }
  }
}
