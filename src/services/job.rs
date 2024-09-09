use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Job;
use crate::entities::job;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_job(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  person_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Job::delete_by_id((
        organization_id,
        person_id,
        created_at,
      ))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("job"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let job = Job::find_by_id((
        organization_id,
        person_id,
        created_at,
      )).one(db).await?;
      if job.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("job"))).into());
      }

      let mut job: job::ActiveModel = job.unwrap().into();
      job.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      job.update(db).await?;
      Ok(())
    },
  }
}
