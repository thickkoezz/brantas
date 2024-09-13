use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Phone, phone};
use crate::dtos::phone::PhoneResponse;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;

pub async fn delete_phone(deletion_mode: DeletionMode, phone: String) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = Phone::delete_by_id(phone).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("person"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let phone = Phone::find_by_id(phone).one(db).await?;
      if phone.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("phone"))).into());
      }

      let mut phone: phone::ActiveModel = phone.unwrap().into();
      phone.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      phone.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_phone(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<PhoneResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let phones = Phone::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = phones.into_iter()
        .map(|phone: phone::Model| PhoneResponse::from(phone)).collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let phones = Phone::find().all(db).await?;
      let res = phones.into_iter()
        .map(|phone: phone::Model| PhoneResponse::from(phone)).collect::<Vec<_>>();
      Ok(res)
    }
  }
}
