use crate::entities::user_account;
use salvo::prelude::{Extractible, ToSchema};
use sea_orm::prelude::{DateTimeWithTimeZone, Decimal};
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = Uuid;

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct UserAccountDTO {
  pub id: Uuid,
  pub owner_id: Uuid,
  pub email: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub picture: Option<String>,
  #[serde(skip_serializing)]
  pub password: Option<String>,
  #[serde(skip_serializing)]
  pub salt: Option<String>,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub balance: Decimal,
  pub is_super_admin: bool,
  pub r#type: String,
  pub provider: String,
  pub provider_account_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub refresh_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub access_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub token_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub scope: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id_token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub session_state: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expires_at: Option<DateTimeWithTimeZone>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub refresh_token_expires_in: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub token: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exp: Option<i64>,
}

impl UserAccountDTO {
  pub fn new() -> Self {
    Self {
      id: Uuid::new_v4(),
      created_at: DateTimeWithTimeZone::from(Local::now()),
      ..Default::default()
    }
  }

  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    self.id
  }

  pub fn set_id(&mut self, id: ID) -> &mut Self {
    self.id = id;
    self
  }

  pub fn set_owner_id(&mut self, val: Uuid) -> &mut Self {
    self.owner_id = val;
    self
  }

  pub fn set_email(&mut self, val: String) -> &mut Self {
    self.email = val;
    self
  }

  pub fn set_username(&mut self, val: Option<String>) -> &mut Self {
    self.username = val;
    self
  }

  pub fn set_picture(&mut self, val: Option<String>) -> &mut Self {
    self.picture = val;
    self
  }

  pub fn set_password(&mut self, val: Option<String>) -> &mut Self {
    self.password = val;
    self
  }

  pub fn set_salt(&mut self, val: Option<String>) -> &mut Self {
    self.salt = val;
    self
  }

  pub fn set_created_at(&mut self, val: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = val;
    self
  }

  pub fn set_updated_at(&mut self, val: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.updated_at = val;
    self
  }

  pub fn set_deleted_at(&mut self, val: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.deleted_at = val;
    self
  }

  pub fn set_balance(&mut self, val: Decimal) -> &mut Self {
    self.balance = val;
    self
  }

  pub fn set_is_super_admin(&mut self, val: bool) -> &mut Self {
    self.is_super_admin = val;
    self
  }

  pub fn set_type(&mut self, val: String) -> &mut Self {
    self.r#type = val;
    self
  }

  pub fn set_provider(&mut self, val: String) -> &mut Self {
    self.provider = val;
    self
  }

  pub fn set_provider_account_id(&mut self, val: String) -> &mut Self {
    self.provider_account_id = val;
    self
  }

  pub fn set_refresh_token(&mut self, val: Option<String>) -> &mut Self {
    self.refresh_token = val;
    self
  }

  pub fn set_access_token(&mut self, val: Option<String>) -> &mut Self {
    self.access_token = val;
    self
  }

  pub fn set_token_type(&mut self, val: Option<String>) -> &mut Self {
    self.token_type = val;
    self
  }

  pub fn set_scope(&mut self, val: Option<String>) -> &mut Self {
    self.scope = val;
    self
  }

  pub fn set_id_token(&mut self, val: Option<String>) -> &mut Self {
    self.id_token = val;
    self
  }

  pub fn set_session_state(&mut self, val: Option<String>) -> &mut Self {
    self.session_state = val;
    self
  }

  pub fn set_expires_at(&mut self, val: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.expires_at = val;
    self
  }

  pub fn set_refresh_token_expires_in(&mut self, val: Option<i32>) -> &mut Self {
    self.refresh_token_expires_in = val;
    self
  }

  pub fn set_token(&mut self, v: Option<String>) -> &mut Self {
    self.token = v;
    self
  }

  pub fn set_exp(&mut self, v: Option<i64>) -> &mut Self {
    self.exp = v;
    self
  }
}

impl From<user_account::Model> for UserAccountDTO {
  fn from(m: user_account::Model) -> Self {
    Self {
      id: m.id,
      owner_id: m.owner_id,
      email: m.email,
      username: m.username,
      picture: m.picture,
      password: m.password,
      salt: m.salt,
      created_at: m.created_at,
      updated_at: m.updated_at,
      deleted_at: m.deleted_at,
      balance: m.balance,
      is_super_admin: m.is_super_admin,
      r#type: m.r#type,
      provider: m.provider,
      provider_account_id: m.provider_account_id,
      refresh_token: m.refresh_token,
      access_token: m.access_token,
      token_type: m.token_type,
      scope: m.scope,
      id_token: m.id_token,
      session_state: m.session_state,
      expires_at: m.expires_at,
      refresh_token_expires_in: m.refresh_token_expires_in,
      ..Default::default()
    }
  }
}

impl From<user_account::ActiveModel> for UserAccountDTO {
  fn from(m: user_account::ActiveModel) -> Self {
    Self {
      id: m.id.unwrap(),
      owner_id: m.owner_id.unwrap(),
      email: m.email.unwrap(),
      username: m.username.unwrap(),
      picture: m.picture.unwrap(),
      password: m.password.unwrap(),
      salt: m.salt.unwrap(),
      created_at: m.created_at.unwrap(),
      updated_at: m.updated_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
      balance: m.balance.unwrap(),
      is_super_admin: m.is_super_admin.unwrap(),
      r#type: m.r#type.unwrap(),
      provider: m.provider.unwrap(),
      provider_account_id: m.provider_account_id.unwrap(),
      refresh_token: m.refresh_token.unwrap(),
      access_token: m.access_token.unwrap(),
      token_type: m.token_type.unwrap(),
      scope: m.scope.unwrap(),
      id_token: m.id_token.unwrap(),
      session_state: m.session_state.unwrap(),
      expires_at: m.expires_at.unwrap(),
      refresh_token_expires_in: m.refresh_token_expires_in.unwrap(),
      ..Default::default()
    }
  }
}
