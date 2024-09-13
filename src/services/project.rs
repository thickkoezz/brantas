use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Project, project};
use crate::dtos::project::ProjectResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_project(
  deletion_mode: DeletionMode,
  person_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = Project::delete_by_id((person_id, created_at)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("project"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let project = Project::find_by_id((person_id, created_at)).one(db).await?;
      if project.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("project"))).into());
      }

      let mut project: project::ActiveModel = project.unwrap().into();
      project.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      project.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_project(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<ProjectResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let projects = Project::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = projects.into_iter()
        .map(|project: project::Model| ProjectResponse::from(project)).collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let projects = Project::find().all(db).await?;
      let res = projects.into_iter()
        .map(|project: project::Model| ProjectResponse::from(project)).collect::<Vec<_>>();
      Ok(res)
    }
  }
}
