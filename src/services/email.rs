use sea_orm::EntityTrait;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Email;
use super::DeletionMode;

pub async fn delete_email(
  deletion_mode: DeletionMode,
  email: String
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      Email::delete_by_id(email).exec(db).await?;
      Ok(())
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
