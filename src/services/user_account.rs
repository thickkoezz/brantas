use argon2::password_hash::SaltString;
use crate::{
  app_writer::AppResult,
  db::DB,
  dtos::user::{
    UserAddRequest, UserLoginRequest, UserLoginResponse, UserResponse,
    UserUpdateRequest,
  },
  middleware::jwt::get_token,
  entities::{prelude::UserAccount, user_account},
  utils::rand_utils,
};
use sea_orm::{EntityTrait, Set, ActiveModelTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;
use rust_i18n::t;

pub async fn add_user_account(req: UserAddRequest) -> AppResult<UserResponse> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let salt = SaltString::generate(rand::thread_rng());
  let model = user_account::ActiveModel {
    id: Set(Uuid::new_v4()),
    owner_id: Default::default(),
    email: Default::default(),
    username: Set(Option::from(req.username.clone())),
    picture: Default::default(),
    password: Set(Option::from(rand_utils::hash_password(req.password, salt.clone()).await?)),
    salt: Set(Option::from(salt.to_string())),
    created_at: Default::default(),
    updated_at: Default::default(),
    deleted_at: Default::default(),
    balance: Default::default(),
    is_super_admin: Default::default(),
    r#type: Default::default(),
    provider: Default::default(),
    provider_account_id: Default::default(),
    refresh_token: Default::default(),
    access_token: Default::default(),
    token_type: Default::default(),
    scope: Default::default(),
    id_token: Default::default(),
    session_state: Default::default(),
    expires_at: Default::default(),
    refresh_token_expires_in: Default::default(),
  };
  let user = UserAccount::insert(model).exec(db).await?;
  Ok(UserResponse {
    id: String::from(user.last_insert_id),
    username: req.username,
  })
}

pub async fn login(req: UserLoginRequest) -> AppResult<UserLoginResponse> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let user = UserAccount::find().filter(user_account::Column::Username.eq(req.username)).one(db).await?;
  if user.is_none() {
    return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user"))).into());
  }
  let user = user.unwrap();
  if let None = user.password {
    Err(anyhow::anyhow!(t!("x_not_set", x = t!("password"))).into())
  } else {
    if rand_utils::verify_password(req.password, user.password.unwrap())
      .await
      .is_err()
    {
      return Err(anyhow::anyhow!(t!("incorrect_x", x = t!("password"))).into());
    }
    let (token, exp) = get_token(user.username.clone().unwrap(), user.id.to_string().clone())?;
    let res = UserLoginResponse {
      id: user.id.to_string(),
      username: user.username.unwrap(),
      token,
      exp,
    };
    Ok(res)
  }
}

pub async fn update_user_account(req: UserUpdateRequest) -> AppResult<UserResponse> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;

  let id = Uuid::try_parse(req.id.as_str()).unwrap();
  let user = UserAccount::find_by_id(id).one(db).await?;
  if user.is_none() {
    return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user"))).into());
  }
  let mut user: user_account::ActiveModel = user.unwrap().into();

  let salt = SaltString::from_b64(user.salt.clone().unwrap().unwrap().as_str()).unwrap();
  let password = rand_utils::hash_password(req.password, salt.clone()).await?;
  user.username = Set(Option::from(req.username.to_owned()));
  user.password = Set(Option::from(password));

  let user: user_account::Model = user.update(db).await?;

  Ok(UserResponse {
    id: user.id.to_string(),
    username: user.username.unwrap(),
  })
}

pub async fn delete_user_account(id: Uuid) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  UserAccount::delete_by_id(id).exec(db).await?;
  Ok(())
}

pub async fn get_user_accounts() -> AppResult<Vec<UserResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let users = UserAccount::find().all(db).await?;
  let res = users
    .into_iter()
    .map(|user| UserResponse {
      id: String::from(user.id),
      username: user.username.unwrap(),
    })
    .collect::<Vec<_>>();
  Ok(res)
}
