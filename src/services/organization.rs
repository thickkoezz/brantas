use sea_orm::EntityTrait;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Organization;
use super::DeletionMode;

pub async fn delete_organization(
  deletion_mode: DeletionMode,
  id: Uuid,
) -> AppResult<()> {
  let db = DB.get().ok_or(
    anyhow::anyhow!(t!("database_connection_failed"))
  )?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Organization::delete_by_id(id).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("organization"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
