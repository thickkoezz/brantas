use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::job_project::JobProjectDTO;
use crate::entities::{job_project, prelude::JobProject};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_job_project(req: JobProjectDTO) -> AppResult<JobProjectDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = job_project::ActiveModel {
    organization_id: Set(req.organization_id),
    person_id: Set(req.person_id),
    job_created_at: Set(req.job_created_at),
    project_created_at: Set(req.project_created_at),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    deleted_at: Set(None),
    description: Set(req.description),
  };
  let job_project = JobProject::insert(model.clone()).exec(db).await?;
  Ok(JobProjectDTO {
    organization_id: job_project.last_insert_id.0,
    person_id: job_project.last_insert_id.1,
    job_created_at: job_project.last_insert_id.2,
    project_created_at: job_project.last_insert_id.3,
    ..JobProjectDTO::from(model)
  })
}

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
    }
    DeletionMode::Soft => {
      let job_project = JobProject::find_by_id((
        organization_id,
        person_id,
        job_created_at,
        project_created_at,
      ))
      .one(db)
      .await?;
      if job_project.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("chat"))).into());
      }

      let mut job_project: job_project::ActiveModel = job_project.unwrap().into();
      job_project.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      job_project.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_job_project(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<JobProjectDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let job_projects = JobProject::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = job_projects
        .into_iter()
        .map(|job_project: job_project::Model| JobProjectDTO::from(job_project))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let job_projects = JobProject::find().all(db).await?;
      let res = job_projects
        .into_iter()
        .map(|job_project: job_project::Model| JobProjectDTO::from(job_project))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
