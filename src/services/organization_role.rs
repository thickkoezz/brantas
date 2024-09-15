use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::organization_role::{OrganizationRoleAddRequest, OrganizationRoleResponse};
use crate::entities::{organization_role, prelude::OrganizationRole};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_organization_role(
  req: OrganizationRoleAddRequest,
) -> AppResult<OrganizationRoleResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = organization_role::ActiveModel {
    organization_id: Set(req.organization_id),
    name: Set(req.name),
    description: Set(req.description),
    extra_info: Set(req.extra_info),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
  };
  let organization_role = OrganizationRole::insert(model.clone()).exec(db).await?;
  Ok(OrganizationRoleResponse {
    organization_id: organization_role.last_insert_id.0,
    name: organization_role.last_insert_id.1,
    ..OrganizationRoleResponse::from(model)
  })
}

pub async fn delete_organization_role(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  name: String,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = OrganizationRole::delete_by_id((organization_id, name))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("role"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let organization_role = OrganizationRole::find_by_id((organization_id, name))
        .one(db)
        .await?;
      if organization_role.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("role"))).into());
      }

      let mut organization_role: organization_role::ActiveModel = organization_role.unwrap().into();
      organization_role.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      organization_role.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_organization_role(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<OrganizationRoleResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let organization_roles = OrganizationRole::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = organization_roles
        .into_iter()
        .map(|organization_role: organization_role::Model| {
          OrganizationRoleResponse::from(organization_role)
        })
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let organization_roles = OrganizationRole::find().all(db).await?;
      let res = organization_roles
        .into_iter()
        .map(|organization_role: organization_role::Model| {
          OrganizationRoleResponse::from(organization_role)
        })
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
