use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::OrganizationAddress;
use crate::entities::organization_address;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_organization_address(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  city_id: i32,
  department_id: Uuid,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = OrganizationAddress::delete_by_id((
        organization_id,
        city_id,
        department_id,
      ))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("address"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let organization_address = OrganizationAddress::find_by_id((
        organization_id,
        city_id,
        department_id,
      ))
        .one(db)
        .await?;
      if organization_address.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("address"))).into());
      }

      let mut organization_address: organization_address::ActiveModel
        = organization_address.unwrap().into();
      organization_address.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      organization_address.update(db).await?;
      Ok(())
    }
  }
}
