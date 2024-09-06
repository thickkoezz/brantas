use sea_orm::EntityTrait;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::OrganizationAdministrator;
use super::DeletionMode;

pub async fn delete_organization_administrator(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  administrator_id: Uuid,
  department_id: Uuid,
) -> AppResult<()> {
  let db = DB.get().ok_or(
    anyhow::anyhow!(t!("database_connection_failed"))
  )?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = OrganizationAdministrator::delete_by_id((
        organization_id,
        administrator_id,
        department_id,
      )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("organization_administrator"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
