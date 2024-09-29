use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::organization_administrator::OrganizationAdministratorDTO;
use crate::entities::{organization_administrator, prelude::OrganizationAdministrator};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_organization_administrator(
  req: OrganizationAdministratorDTO,
) -> AppResult<OrganizationAdministratorDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = organization_administrator::ActiveModel {
    organization_id: Set(req.organization_id),
    administrator_id: Set(req.administrator_id),
    department_id: Set(req.department_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    deleted_at: Set(None),
  };
  let organization_administrator = OrganizationAdministrator::insert(model.clone())
    .exec(db)
    .await?;
  Ok(OrganizationAdministratorDTO {
    organization_id: organization_administrator.last_insert_id.0,
    administrator_id: organization_administrator.last_insert_id.1,
    ..OrganizationAdministratorDTO::from(model)
  })
}

pub async fn delete_organization_administrator(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  administrator_id: Uuid,
  department_id: Uuid,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result =
        OrganizationAdministrator::delete_by_id((organization_id, administrator_id, department_id))
          .exec(db)
          .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("administrator"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let organization_administrator =
        OrganizationAdministrator::find_by_id((organization_id, administrator_id, department_id))
          .one(db)
          .await?;
      if organization_administrator.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("administrator"))).into());
      }

      let mut organization_administrator: organization_administrator::ActiveModel =
        organization_administrator.unwrap().into();
      organization_administrator.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      organization_administrator.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_organization_administrator(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<OrganizationAdministratorDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let organization_administrators = OrganizationAdministrator::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = organization_administrators
        .into_iter()
        .map(
          |organization_administrator: organization_administrator::Model| {
            OrganizationAdministratorDTO::from(organization_administrator)
          },
        )
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let organization_administrators = OrganizationAdministrator::find().all(db).await?;
      let res = organization_administrators
        .into_iter()
        .map(
          |organization_administrator: organization_administrator::Model| {
            OrganizationAdministratorDTO::from(organization_administrator)
          },
        )
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
