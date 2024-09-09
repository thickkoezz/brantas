use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::OrganizationRole;
use crate::entities::organization_role;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_organization_role(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  name: String,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = OrganizationRole::delete_by_id((
        organization_id,
        name,
      ))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("role"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let organization_role = OrganizationRole::find_by_id((
        organization_id,
        name,
      ))
        .one(db)
        .await?;
      if organization_role.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("role"))).into());
      }

      let mut organization_role: organization_role::ActiveModel
        = organization_role.unwrap().into();
      organization_role.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      organization_role.update(db).await?;
      Ok(())
    },
  }
}
