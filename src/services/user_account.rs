use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::UserAccount, user_account};
use crate::dtos::user_account::{
  UserAccountAddRequest, UserAccountLoginRequest, UserAccountLoginResponse, UserAccountResponse,
  UserAccountUpdateRequest,
};
use crate::middleware::jwt::get_token;
use crate::utils::rand_utils;
use argon2::password_hash::SaltString;
use rust_i18n::t;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn add_user_account(
  req: UserAccountAddRequest
) -> AppResult<UserAccountResponse> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let salt = SaltString::generate(rand::thread_rng());
  let model = user_account::ActiveModel {
    id: Set(Uuid::new_v4()),
    owner_id: Set(req.owner_id),
    email: Set(req.email.clone()),
    username: Set(Option::from(req.username.clone())),
    picture: Set(req.picture.clone()),
    password: Set(Option::from(
      rand_utils::hash_password(req.password.unwrap(), salt.clone()).await?,
    )),
    salt: Set(Option::from(salt.to_string())),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Default::default(),
    deleted_at: Default::default(),
    balance: Set(req.balance.clone()),
    is_super_admin: Set(req.is_super_admin),
    r#type: Set(req.r#type.clone()),
    provider: Set(req.provider.clone()),
    provider_account_id: Set(req.provider_account_id.clone()),
    refresh_token: Set(req.refresh_token.clone()),
    access_token: Set(req.access_token.clone()),
    token_type: Set(req.token_type.clone()),
    scope: Set(req.scope.clone()),
    id_token: Set(req.id_token.clone()),
    session_state: Set(req.session_state.clone()),
    expires_at: Set(req.expires_at),
    refresh_token_expires_in: Set(req.refresh_token_expires_in),
  };
  let user = UserAccount::insert(model.clone()).exec(db).await?;
  Ok(UserAccountResponse {
    id: user.last_insert_id,
    ..UserAccountResponse::from(req)
  })
}

pub async fn login(
  req: UserAccountLoginRequest
) -> AppResult<UserAccountLoginResponse> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let user = UserAccount::find().filter(user_account::Column::Username.eq(req.username))
    .one(db).await?;
  if user.is_none() {
    return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user_account"))).into());
  }
  let user = user.unwrap();
  if user.password.is_none() {
    return Err(anyhow::anyhow!(t!("x_not_set", x = t!("password"))).into());
  }
  let password = user.password.unwrap();
  if rand_utils::verify_password(req.password, password).await.is_err()
  {
    return Err(anyhow::anyhow!(t!("incorrect_x", x = t!("password"))).into());
  }
  let (token, exp) = get_token(
    user.username.clone().unwrap_or_default(), user.id.to_string().clone(),
  )?;
  let res = UserAccountLoginResponse {
    id: user.id, username: user.username.unwrap_or_default(), token, exp,
  };
  Ok(res)
}

pub async fn update_user_account(
  req: UserAccountUpdateRequest
) -> AppResult<UserAccountResponse> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let user = UserAccount::find_by_id(req.id).one(db).await?;
  if user.is_none() {
    return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user_account"))).into());
  }
  let mut user: user_account::ActiveModel = user.unwrap().into();

  let salt = SaltString::from_b64(user.salt.clone().unwrap().unwrap_or_default().as_str())
    .unwrap();
  let password = rand_utils::hash_password(req.password.unwrap_or_default(), salt, )
    .await?;

  user.owner_id = Set(req.owner_id);
  user.email = Set(req.email);
  user.username = Set(Option::from(req.username.to_owned()));
  user.picture = Set(req.picture);
  user.password = Set(Option::from(password));
  user.updated_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
  user.r#type = Set(req.r#type);
  user.provider = Set(req.provider);
  user.provider_account_id = Set(req.provider_account_id);
  user.refresh_token = Set(req.refresh_token);
  user.access_token = Set(req.access_token);
  user.token_type = Set(req.token_type);
  user.scope = Set(req.scope);
  user.id_token = Set(req.id_token);
  user.session_state = Set(req.session_state);
  user.expires_at = Set(req.expires_at);
  user.refresh_token_expires_in = Set(req.refresh_token_expires_in);

  let user: user_account::Model = user.update(db).await?;
  Ok(UserAccountResponse::from(user))
}

pub async fn delete_user_account(
  deletion_mode: DeletionMode,
  id: Uuid,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = UserAccount::delete_by_id(id).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("user_account"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let user = UserAccount::find_by_id(id).one(db).await?;
      if user.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user_account"))).into());
      }

      let mut user: user_account::ActiveModel = user.unwrap().into();
      user.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      user.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_user_accounts(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<UserAccountResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let user_accounts = UserAccount::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = user_accounts.into_iter()
        .map(|user: user_account::Model| UserAccountResponse::from(user))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let user_accounts = UserAccount::find().all(db).await?;
      let res = user_accounts.into_iter()
        .map(|user_account: user_account::Model| UserAccountResponse::from(user_account))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
