use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::job_skill::JobSkillDTO;
use crate::entities::{job_skill, prelude::JobSkill};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_job_skill(req: JobSkillDTO) -> AppResult<JobSkillDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = job_skill::ActiveModel {
    organization_id: Set(req.organization_id),
    person_id: Set(req.person_id),
    job_created_at: Set(req.job_created_at),
    skill_created_at: Set(req.skill_created_at),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    deleted_at: Set(None),
    description: Set(req.description),
  };
  let job_skill = JobSkill::insert(model.clone()).exec(db).await?;
  Ok(JobSkillDTO {
    organization_id: job_skill.last_insert_id.0,
    person_id: job_skill.last_insert_id.1,
    job_created_at: job_skill.last_insert_id.2,
    skill_created_at: job_skill.last_insert_id.3,
    ..JobSkillDTO::from(model)
  })
}

pub async fn delete_job_skill(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  person_id: Uuid,
  job_created_at: DateTimeWithTimeZone,
  skill_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result =
        JobSkill::delete_by_id((organization_id, person_id, job_created_at, skill_created_at))
          .exec(db)
          .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("skill"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let job_skill =
        JobSkill::find_by_id((organization_id, person_id, job_created_at, skill_created_at))
          .one(db)
          .await?;
      if job_skill.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("skill"))).into());
      }

      let mut job_skill: job_skill::ActiveModel = job_skill.unwrap().into();
      job_skill.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      job_skill.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_job_skill(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<JobSkillDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let job_skills = JobSkill::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = job_skills
        .into_iter()
        .map(|job_skill: job_skill::Model| JobSkillDTO::from(job_skill))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let job_skills = JobSkill::find().all(db).await?;
      let res = job_skills
        .into_iter()
        .map(|job_skill: job_skill::Model| JobSkillDTO::from(job_skill))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
