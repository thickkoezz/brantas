use sea_orm::EntityTrait;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Email;
use super::DeletionMode;

pub async fn delete_email(
  deletion_mode: DeletionMode,
  email: String,
) -> AppResult<()> {
  let db = DB.get().ok_or(
    anyhow::anyhow!(t!("database_connection_failed"))
  )?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Email::delete_by_id(email).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("email"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
