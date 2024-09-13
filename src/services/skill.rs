use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Skill, skill};
use crate::dtos::skill::SkillResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_skill(
  deletion_mode: DeletionMode,
  person_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = Skill::delete_by_id((person_id, created_at,)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("skill"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let skill = Skill::find_by_id((person_id, created_at,)).one(db).await?;
      if skill.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("skill"))).into());
      }

      let mut skill: skill::ActiveModel = skill.unwrap().into();
      skill.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      skill.update(db).await?;
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
