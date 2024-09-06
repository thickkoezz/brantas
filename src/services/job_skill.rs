use sea_orm::EntityTrait;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::JobSkill;
use super::DeletionMode;

pub async fn delete_job_skill(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  person_id: Uuid,
  job_created_at: DateTimeWithTimeZone,
  skill_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB.get().ok_or(
    anyhow::anyhow!(t!("database_connection_failed")
        ))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = JobSkill::delete_by_id((
        organization_id,
        person_id,
        job_created_at,
        skill_created_at
      )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("job_skill"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      todo!()
    }
  }
}
