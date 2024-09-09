use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Phone;
use crate::entities::phone;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;

pub async fn delete_phone(deletion_mode: DeletionMode, phone: String) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = Phone::delete_by_id(phone).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("person"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let phone = Phone::find_by_id(phone).one(db).await?;
      if phone.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("phone"))).into());
      }

      let mut phone: phone::ActiveModel = phone.unwrap().into();
      phone.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      phone.update(db).await?;
      Ok(())
    },
  }
}
