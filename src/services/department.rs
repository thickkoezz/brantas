use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::department::{DepartmentAddRequest, DepartmentResponse};
use crate::entities::{department, prelude::Department};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_department(req: DepartmentAddRequest) -> AppResult<DepartmentResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = department::ActiveModel {
    id: Set(Uuid::new_v4()),
    organization_id: Set(req.organization_id),
    name: Set(req.name),
    abbreviation: Set(req.abbreviation),
    description: Set(req.description),
    dob: Set(req.dob),
    is_dead: Set(req.is_dead),
    dead_at: Set(req.dead_at),
    extra_info: Set(req.extra_info),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
    parent_id: Set(req.parent_id),
    logo: Set(req.logo),
  };
  let department = Department::insert(model.clone()).exec(db).await?;
  Ok(DepartmentResponse {
    id: department.last_insert_id,
    ..DepartmentResponse::from(model)
  })
}

pub async fn delete_department(deletion_mode: DeletionMode, id: Uuid) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Department::delete_by_id(id).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("department"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let department = Department::find_by_id(id).one(db).await?;
      if department.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("department"))).into());
      }

      let mut department: department::ActiveModel = department.unwrap().into();
      department.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      department.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_department(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<DepartmentResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let departments = Department::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = departments
        .into_iter()
        .map(|department: department::Model| DepartmentResponse::from(department))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let departments = Department::find().all(db).await?;
      let res = departments
        .into_iter()
        .map(|department: department::Model| DepartmentResponse::from(department))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
