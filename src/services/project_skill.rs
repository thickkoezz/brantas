use sea_orm::EntityTrait;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::ProjectSkill;
use super::DeletionMode;

pub async fn delete_project_skill(
  deletion_mode: DeletionMode,
  person_id: Uuid,
  project_created_at: DateTimeWithTimeZone,
  skill_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(
        anyhow::anyhow!(t!("database_connection_failed")
        ))?;
      let result = ProjectSkill::delete_by_id((
        person_id,
        project_created_at,
        skill_created_at
      )).exec(db).await?;
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
