use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::OrganizationAdministrator;
use crate::entities::organization_administrator;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_organization_administrator(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  administrator_id: Uuid,
  department_id: Uuid,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result =
        OrganizationAdministrator::delete_by_id((
          organization_id,
          administrator_id,
          department_id,
        ))
          .exec(db)
          .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("administrator"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let organization_administrator = OrganizationAdministrator::find_by_id((
        organization_id,
        administrator_id,
        department_id,
      ))
        .one(db)
        .await?;
      if organization_administrator.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("administrator"))).into());
      }

      let mut organization_administrator: organization_administrator::ActiveModel
        = organization_administrator.unwrap().into();
      organization_administrator.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      organization_administrator.update(db).await?;
      Ok(())
    },
  }
}
