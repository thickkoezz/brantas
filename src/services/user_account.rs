use argon2::password_hash::SaltString;
use crate::{
  app_writer::AppResult,
  db::DB,
  dtos::user_account::{
    UserAccountAddRequest, UserAccountLoginRequest, UserAccountLoginResponse, UserAccountResponse,
    UserAccountUpdateRequest,
  },
  middleware::jwt::get_token,
  entities::{prelude::UserAccount, user_account},
  utils::rand_utils,
};
use sea_orm::{EntityTrait, Set, ActiveModelTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;
use rust_i18n::t;

pub async fn add_user_account(req: UserAccountAddRequest) -> AppResult<UserAccountResponse> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let salt = SaltString::generate(rand::thread_rng());
  let model = user_account::ActiveModel {
    id: Set(Uuid::new_v4()),
    owner_id: Set(req.owner_id),
    email: Set(req.email.clone()),
    username: Set(Option::from(req.username.clone())),
    picture: Set(req.picture.clone()),
    password: Set(Option::from(rand_utils::hash_password(req.password.unwrap(), salt.clone()).await?)),
    salt: Set(Option::from(salt.to_string())),
    created_at: Default::default(),
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
    owner_id: req.owner_id,
    email: req.email,
    username: req.username,
    picture: req.picture,
    salt: req.salt,
    created_at: model.created_at.unwrap(),
    updated_at: None,
    deleted_at: None,
    balance: req.balance,
    is_super_admin: req.is_super_admin,
    r#type: req.r#type,
    provider: req.provider,
    provider_account_id: req.provider_account_id,
    refresh_token: req.refresh_token,
    access_token: req.access_token,
    token_type: req.token_type,
    scope: req.scope,
    id_token: req.id_token,
    session_state: req.session_state,
    expires_at: req.expires_at,
    refresh_token_expires_in: req.refresh_token_expires_in,
  })
}

pub async fn login(req: UserAccountLoginRequest) -> AppResult<UserAccountLoginResponse> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let user = UserAccount::find().filter(user_account::Column::Username.eq(req.username)).one(db).await?;
  if user.is_none() {
    return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user"))).into());
  }
  let user = user.unwrap();
  if let None = user.password {
    Err(anyhow::anyhow!(t!("x_not_set", x = t!("password"))).into())
  } else {
    if rand_utils::verify_password(req.password, user.password.unwrap_or_default())
      .await
      .is_err()
    {
      return Err(anyhow::anyhow!(t!("incorrect_x", x = t!("password"))).into());
    }
    let (token, exp) = get_token(user.username.clone().unwrap_or_default(), user.id.to_string().clone())?;
    let res = UserAccountLoginResponse {
      id: user.id,
      username: user.username.unwrap_or_default(),
      token,
      exp,
    };
    Ok(res)
  }
}

pub async fn update_user_account(req: UserAccountUpdateRequest) -> AppResult<UserAccountResponse> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let user = UserAccount::find_by_id(req.id).one(db).await?;
  if user.is_none() {
    return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user"))).into());
  }
  let mut user: user_account::ActiveModel = user.unwrap().into();

  let salt = SaltString::from_b64(user.salt.clone().unwrap().unwrap_or_default().as_str()).unwrap();
  let password = rand_utils::hash_password(req.password.unwrap_or_default(), salt.clone()).await?;
  user.owner_id = Set(req.owner_id);
  user.email = Set(req.email);
  user.username = Set(Option::from(req.username.to_owned()));
  user.picture = Set(req.picture);
  user.password = Set(Option::from(password));
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
  Ok(UserAccountResponse {
    id: user.id,
    owner_id: user.owner_id,
    email: user.email,
    username: user.username,
    picture: user.picture,
    salt: user.salt,
    created_at: user.created_at,
    updated_at: user.updated_at,
    deleted_at: user.deleted_at,
    balance: user.balance,
    is_super_admin: user.is_super_admin,
    r#type: user.r#type,
    provider: user.provider,
    provider_account_id: user.provider_account_id,
    refresh_token: user.refresh_token,
    access_token: user.access_token,
    token_type: user.token_type,
    scope: user.scope,
    id_token: user.id_token,
    session_state: user.session_state,
    expires_at: user.expires_at,
    refresh_token_expires_in: user.refresh_token_expires_in,
  })
}

pub async fn delete_user_account(id: Uuid) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  UserAccount::delete_by_id(id).exec(db).await?;
  Ok(())
}

pub async fn get_user_accounts() -> AppResult<Vec<UserAccountResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let users = UserAccount::find().all(db).await?;
  let res = users
    .into_iter()
    .map(|user| UserAccountResponse {
      id: user.id,
      owner_id: user.owner_id,
      email: user.email,
      username: user.username,
      picture: user.picture,
      salt: user.salt,
      created_at: user.created_at,
      updated_at: user.updated_at,
      deleted_at: user.deleted_at,
      balance: user.balance,
      is_super_admin: user.is_super_admin,
      r#type: user.r#type,
      provider: user.provider,
      provider_account_id: user.provider_account_id,
      refresh_token: user.refresh_token,
      access_token: user.access_token,
      token_type: user.token_type,
      scope: user.scope,
      id_token: user.id_token,
      session_state: user.session_state,
      expires_at: user.expires_at,
      refresh_token_expires_in: user.refresh_token_expires_in,
    })
    .collect::<Vec<_>>();
  Ok(res)
}
