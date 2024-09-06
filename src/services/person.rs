use sea_orm::EntityTrait;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Person;
use super::DeletionMode;

pub async fn delete_person(
  deletion_mode: DeletionMode,
  id: Uuid,
) -> AppResult<()> {
  let db = DB.get().ok_or(
    anyhow::anyhow!(t!("database_connection_failed"))
  )?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Person::delete_by_id(id).exec(db).await?;
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
