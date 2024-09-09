use salvo::prelude::{Extractible, ToSchema};
use sea_orm::prelude::{DateTimeWithTimeZone, Decimal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate, Extractible, ToSchema, Default)]
pub struct UserAccountAddRequest {
  pub owner_id: Uuid,
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 5, message = "username length must be greater than 5"))]
  pub username: Option<String>,
  pub picture: Option<String>,
  pub password: Option<String>,
  pub salt: Option<String>,
  pub balance: Decimal,
  pub is_super_admin: bool,
  pub r#type: String,
  pub provider: String,
  pub provider_account_id: String,
  pub refresh_token: Option<String>,
  pub access_token: Option<String>,
  pub token_type: Option<String>,
  pub scope: Option<String>,
  pub id_token: Option<String>,
  pub session_state: Option<String>,
  pub expires_at: Option<DateTimeWithTimeZone>,
  pub refresh_token_expires_in: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema, Default)]
pub struct UserAccountLoginRequest {
  pub username: String,
  pub password: String,
}

#[derive(Debug, Deserialize, Validate, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct UserAccountUpdateRequest {
  pub id: Uuid,
  pub owner_id: Uuid,
  #[validate(email)]
  pub email: String,
  pub username: String,
  pub picture: Option<String>,
  pub password: Option<String>,
  pub salt: Option<String>,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub balance: Decimal,
  pub is_super_admin: bool,
  pub r#type: String,
  pub provider: String,
  pub provider_account_id: String,
  pub refresh_token: Option<String>,
  pub access_token: Option<String>,
  pub token_type: Option<String>,
  pub scope: Option<String>,
  pub id_token: Option<String>,
  pub session_state: Option<String>,
  pub expires_at: Option<DateTimeWithTimeZone>,
  pub refresh_token_expires_in: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct UserAccountResponse {
  pub id: Uuid,
  pub owner_id: Uuid,
  pub email: String,
  pub username: Option<String>,
  pub picture: Option<String>,
  pub salt: Option<String>,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub balance: Decimal,
  pub is_super_admin: bool,
  pub r#type: String,
  pub provider: String,
  pub provider_account_id: String,
  pub refresh_token: Option<String>,
  pub access_token: Option<String>,
  pub token_type: Option<String>,
  pub scope: Option<String>,
  pub id_token: Option<String>,
  pub session_state: Option<String>,
  pub expires_at: Option<DateTimeWithTimeZone>,
  pub refresh_token_expires_in: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct UserAccountLoginResponse {
  pub id: Uuid,
  pub username: String,
  pub token: String,
  pub exp: i64,
}
