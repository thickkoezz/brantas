use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::project_skill::ProjectSkillDTO;
use crate::entities::{prelude::ProjectSkill, project_skill};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_project_skill(req: ProjectSkillDTO) -> AppResult<ProjectSkillDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = project_skill::ActiveModel {
    person_id: Set(req.person_id),
    project_created_at: Set(req.project_created_at),
    skill_created_at: Set(req.skill_created_at),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    deleted_at: Set(None),
    description: Set(req.description),
  };
  let project_skill = ProjectSkill::insert(model.clone()).exec(db).await?;
  Ok(ProjectSkillDTO {
    person_id: project_skill.last_insert_id.0,
    project_created_at: project_skill.last_insert_id.1,
    skill_created_at: project_skill.last_insert_id.2,
    ..ProjectSkillDTO::from(model)
  })
}

pub async fn delete_project_skill(
  deletion_mode: DeletionMode,
  person_id: Uuid,
  project_created_at: DateTimeWithTimeZone,
  skill_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = ProjectSkill::delete_by_id((person_id, project_created_at, skill_created_at))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("skill"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let project_skill =
        ProjectSkill::find_by_id((person_id, project_created_at, skill_created_at))
          .one(db)
          .await?;
      if project_skill.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("skill"))).into());
      }

      let mut project_skill: project_skill::ActiveModel = project_skill.unwrap().into();
      project_skill.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      project_skill.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_project_skill(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<ProjectSkillDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let project_skills = ProjectSkill::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = project_skills
        .into_iter()
        .map(|project_skill: project_skill::Model| ProjectSkillDTO::from(project_skill))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let project_skills = ProjectSkill::find().all(db).await?;
      let res = project_skills
        .into_iter()
        .map(|project_skill: project_skill::Model| ProjectSkillDTO::from(project_skill))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
