use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::{Skill, JobSkill};
use crate::entities::{skill, job_skill};
use crate::dtos::skill::SkillResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_job_skill(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  person_id: Uuid,
  job_created_at: DateTimeWithTimeZone,
  skill_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result =
        JobSkill::delete_by_id((
          organization_id, person_id, job_created_at, skill_created_at,
        )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("skill"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let job_skill = JobSkill::find_by_id((
        organization_id, person_id, job_created_at, skill_created_at,
      )).one(db).await?;
      if job_skill.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("skill"))).into());
      }

      let mut job_skill: job_skill::ActiveModel = job_skill.unwrap().into();
      job_skill.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      job_skill.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_skill(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<SkillResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let skills = Skill::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = skills.into_iter()
        .map(|skill: skill::Model| SkillResponse::from(skill)).collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let skills = Skill::find().all(db).await?;
      let res = skills.into_iter()
        .map(|skill: skill::Model| SkillResponse::from(skill)).collect::<Vec<_>>();
      Ok(res)
    }
  }
}
