use crate::entities::session::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = Uuid;

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct SessionDTO {
  pub id: Uuid,
  pub session_token: String,
  pub user_account_id: Uuid,
  pub expires: DateTimeWithTimeZone,
}

impl SessionDTO {
  pub fn set_id(&mut self, v: Uuid) -> &mut Self {
    self.id = v;
    self
  }

  pub fn set_session_token(&mut self, v: String) -> &mut Self {
    self.session_token = v;
    self
  }

  pub fn set_user_account_id(&mut self, v: Uuid) -> &mut Self {
    self.user_account_id = v;
    self
  }

  pub fn set_expires(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.expires = v;
    self
  }
}

impl From<Model> for SessionDTO {
  fn from(m: Model) -> Self {
    Self {
      id: m.id,
      session_token: m.session_token,
      user_account_id: m.user_account_id,
      expires: m.expires,
    }
  }
}

impl From<ActiveModel> for SessionDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      id: m.id.unwrap(),
      session_token: m.session_token.unwrap(),
      user_account_id: m.user_account_id.unwrap(),
      expires: m.expires.unwrap(),
    }
  }
}
