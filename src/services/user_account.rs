use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::user_account::UserAccountDTO;
use crate::entities::{prelude::UserAccount, user_account};
use crate::middleware::jwt::get_token;
use crate::utils::rand_utils;
use argon2::password_hash::SaltString;
use rust_i18n::t;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn add_user_account(req: UserAccountDTO) -> AppResult<UserAccountDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let salt = SaltString::generate(rand::thread_rng());
  let model = user_account::ActiveModel {
    id: Set(Uuid::new_v4()),
    owner_id: Set(req.owner_id),
    email: Set(req.email),
    username: Set(Option::from(req.username)),
    picture: Set(req.picture),
    password: if req.password.is_some() {
      Set(Option::from(
        rand_utils::hash_password(req.password.unwrap(), salt.clone()).await?,
      ))
    } else {
      Set(None)
    },
    salt: Set(Option::from(salt.to_string())),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
    balance: Set(req.balance),
    is_super_admin: Set(req.is_super_admin),
    r#type: Set(req.r#type),
    provider: Set(req.provider),
    provider_account_id: Set(req.provider_account_id),
    refresh_token: Set(req.refresh_token),
    access_token: Set(req.access_token),
    token_type: Set(req.token_type),
    scope: Set(req.scope),
    id_token: Set(req.id_token),
    session_state: Set(req.session_state),
    expires_at: Set(req.expires_at),
    refresh_token_expires_in: Set(req.refresh_token_expires_in),
  };
  let user_account = UserAccount::insert(model.clone()).exec(db).await?;
  Ok(UserAccountDTO {
    id: user_account.last_insert_id,
    ..UserAccountDTO::from(model)
  })
}

pub async fn login(req: UserAccountDTO) -> AppResult<UserAccountDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let user = UserAccount::find()
    .filter(user_account::Column::Username.eq(req.username))
    .one(db)
    .await?;
  if user.is_none() {
    return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user_account"))).into());
  }
  let user_account = user.unwrap();
  if user_account.password.is_none() {
    return Err(anyhow::anyhow!(t!("x_not_set", x = t!("password"))).into());
  }
  let password = user_account.password.unwrap();
  if rand_utils::verify_password(req.password.unwrap(), password)
    .await
    .is_err()
  {
    return Err(anyhow::anyhow!(t!("incorrect_x", x = t!("password"))).into());
  }
  let (token, exp) = get_token(
    user_account.username.clone().unwrap_or_default(),
    user_account.id.to_string().clone(),
  )?;
  let res = UserAccountDTO {
    id: user_account.id,
    username: user_account.username,
    token: Some(token),
    exp: Some(exp),
    ..Default::default()
  };
  Ok(res)
}

pub async fn update_user_account(req: UserAccountDTO) -> AppResult<UserAccountDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let user = UserAccount::find_by_id(req.id).one(db).await?;
  if user.is_none() {
    return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user_account"))).into());
  }
  let mut user_account: user_account::ActiveModel = user.unwrap().into();

  let salt_opt = user_account.salt.clone().unwrap();
  let salt: SaltString;
  if salt_opt.is_some() {
    salt = SaltString::from_b64(salt_opt.unwrap().as_str()).unwrap();
  } else {
    salt = SaltString::generate(rand::thread_rng());
  }
  let password = rand_utils::hash_password(req.password.unwrap_or_default(), salt).await?;

  user_account.owner_id = Set(req.owner_id);
  user_account.email = Set(req.email);
  user_account.username = Set(Option::from(req.username.to_owned()));
  user_account.picture = Set(req.picture);
  user_account.password = Set(Option::from(password));
  user_account.updated_at = Set(Option::from(DateTimeWithTimeZone::from(
    chrono::Local::now(),
  )));
  user_account.r#type = Set(req.r#type);
  user_account.provider = Set(req.provider);
  user_account.provider_account_id = Set(req.provider_account_id);
  user_account.refresh_token = Set(req.refresh_token);
  user_account.access_token = Set(req.access_token);
  user_account.token_type = Set(req.token_type);
  user_account.scope = Set(req.scope);
  user_account.id_token = Set(req.id_token);
  user_account.session_state = Set(req.session_state);
  user_account.expires_at = Set(req.expires_at);
  user_account.refresh_token_expires_in = Set(req.refresh_token_expires_in);

  let user: user_account::Model = user_account.update(db).await?;
  Ok(UserAccountDTO::from(user))
}

pub async fn delete_user_account(deletion_mode: DeletionMode, id: Uuid) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = UserAccount::delete_by_id(id).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("user_account"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let user_account = UserAccount::find_by_id(id).one(db).await?;
      if user_account.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user_account"))).into());
      }

      let mut user_account: user_account::ActiveModel = user_account.unwrap().into();
      user_account.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      user_account.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_user_accounts(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<UserAccountDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let user_accounts = UserAccount::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = user_accounts
        .into_iter()
        .map(|user: user_account::Model| UserAccountDTO::from(user))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let user_accounts = UserAccount::find().all(db).await?;
      let res = user_accounts
        .into_iter()
        .map(|user_account: user_account::Model| UserAccountDTO::from(user_account))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
