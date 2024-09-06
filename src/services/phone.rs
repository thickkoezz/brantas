use sea_orm::EntityTrait;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Phone;
use super::DeletionMode;

pub async fn delete_phone(
  deletion_mode: DeletionMode,
  phone: String,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(
        anyhow::anyhow!(t!("database_connection_failed"))
      )?;
      let result = Phone::delete_by_id(phone).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("person"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
