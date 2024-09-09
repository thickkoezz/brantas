use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::JobProject;
use crate::entities::job_project;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_job_project(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  person_id: Uuid,
  job_created_at: DateTimeWithTimeZone,
  project_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = JobProject::delete_by_id((
        organization_id,
        person_id,
        job_created_at,
        project_created_at,
      ))
      .exec(db)
      .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("job_project"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let job_project = JobProject::find_by_id((
        organization_id,
        person_id,
        job_created_at,
        project_created_at,
      )).one(db).await?;
      if job_project.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("chat"))).into());
      }

      let mut job_project: job_project::ActiveModel = job_project.unwrap().into();
      job_project.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      job_project.update(db).await?;
      Ok(())
    },
  }
}
