use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::ProjectSkill;
use crate::entities::project_skill;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_project_skill(
  deletion_mode: DeletionMode,
  person_id: Uuid,
  project_created_at: DateTimeWithTimeZone,
  skill_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = ProjectSkill::delete_by_id((
        person_id,
        project_created_at,
        skill_created_at,
      ))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("skill"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let project_skill = ProjectSkill::find_by_id((
        person_id,
        project_created_at,
        skill_created_at,
      )).one(db).await?;
      if project_skill.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("skill"))).into());
      }

      let mut project_skill: project_skill::ActiveModel = project_skill.unwrap().into();
      project_skill.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      project_skill.update(db).await?;
      Ok(())
    },
  }
}
