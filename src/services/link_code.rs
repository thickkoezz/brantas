use sea_orm::EntityTrait;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::LinkCode;
use super::DeletionMode;

pub async fn delete_link_code(
  deletion_mode: DeletionMode,
  code: String,
) -> AppResult<()> {
  let db = DB.get().ok_or(
    anyhow::anyhow!(t!("database_connection_failed"))
  )?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = LinkCode::delete_by_id(code).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("link_code"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
