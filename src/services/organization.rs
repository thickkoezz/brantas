use sea_orm::EntityTrait;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Organization;
use super::DeletionMode;

pub async fn delete_organization(
  deletion_mode: DeletionMode,
  id: Uuid
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      Organization::delete_by_id(id).exec(db).await?;
      Ok(())
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
