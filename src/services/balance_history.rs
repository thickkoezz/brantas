use argon2::password_hash::SaltString;
use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::BalanceHistory, balance_history};
use crate::dtos::balance_history::{BalanceHistoryAddRequest, BalanceHistoryResponse};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn add_balance_history(
  req: BalanceHistoryAddRequest
) -> AppResult<BalanceHistoryResponse> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let salt = SaltString::generate(rand::thread_rng());
  let model = balance_history::ActiveModel {
    owner_id: Set(req.owner_id.into()),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    ref_id: Set(req.ref_id.into()),
    amount: Set(req.amount.into()),
    deleted_at: Set(None),
  };
  let user = BalanceHistory::insert(model.clone()).exec(db).await?;
  Ok(BalanceHistoryResponse {
    owner_id: user.last_insert_id.0,
    created_at: user.last_insert_id.1,
    ref_id: user.last_insert_id.2,
    ..BalanceHistoryResponse::from(model)
  })
}

pub async fn delete_balance_history(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
  ref_id: Uuid,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = BalanceHistory::delete_by_id((
        owner_id, created_at, ref_id,
      )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("balance_history"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let balance_history = BalanceHistory::find_by_id((
        owner_id, created_at, ref_id,
      )).one(db).await?;
      if balance_history.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("balance_history"))).into());
      }

      let mut balance_history: balance_history::ActiveModel = balance_history.unwrap().into();
      balance_history.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      balance_history.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_balance_history(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<BalanceHistoryResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let balance_histories = BalanceHistory::find()
        .paginate(db, paginator_option.page_size).fetch_page(paginator_option.page).await?;
      let res = balance_histories.into_iter()
        .map(|balance_history: balance_history::Model|
          BalanceHistoryResponse::from(balance_history))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let balance_histories = BalanceHistory::find().all(db).await?;
      let res = balance_histories.into_iter()
        .map(|balance_history: balance_history::Model|
          BalanceHistoryResponse::from(balance_history))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
