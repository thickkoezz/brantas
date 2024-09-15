use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::job::{JobAddRequest, JobResponse};
use crate::entities::{job, prelude::Job};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_job(req: JobAddRequest) -> AppResult<JobResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = job::ActiveModel {
    organization_id: Set(req.organization_id),
    person_id: Set(req.person_id),
    department_id: Set(req.department_id),
    role: Set(req.role),
    job_description: Set(req.job_description),
    start_at: Set(req.start_at),
    end_at: Set(req.end_at),
    is_head_of_department: Set(req.is_head_of_department),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
  };
  let job = Job::insert(model.clone()).exec(db).await?;
  Ok(JobResponse {
    organization_id: job.last_insert_id.0,
    person_id: job.last_insert_id.1,
    ..JobResponse::from(model)
  })
}

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
      let result = Job::delete_by_id((organization_id, person_id, created_at))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("job"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let job = Job::find_by_id((organization_id, person_id, created_at))
        .one(db)
        .await?;
      if job.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("job"))).into());
      }

      let mut job: job::ActiveModel = job.unwrap().into();
      job.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      job.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_job(paginator_option: Option<PaginatorOption>) -> AppResult<Vec<JobResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let jobs = Job::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = jobs
        .into_iter()
        .map(|job: job::Model| JobResponse::from(job))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let jobs = Job::find().all(db).await?;
      let res = jobs
        .into_iter()
        .map(|job: job::Model| JobResponse::from(job))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
