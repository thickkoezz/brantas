use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::organization::{OrganizationAddRequest, OrganizationResponse};
use crate::entities::{organization, prelude::Organization};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_organization(req: OrganizationAddRequest) -> AppResult<OrganizationResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = organization::ActiveModel {
    id: Set(Uuid::new_v4()),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
    name: Set(req.name),
    abbreviation: Set(req.abbreviation),
    description: Set(req.description),
    dob: Set(req.dob),
    dead_at: Set(req.dead_at),
    extra_info: Set(req.extra_info),
    is_dead: Set(req.is_dead),
    parent_id: Set(req.parent_id),
    logo: Set(req.logo),
  };
  let organization = Organization::insert(model.clone()).exec(db).await?;
  Ok(OrganizationResponse {
    id: organization.last_insert_id,
    ..OrganizationResponse::from(model)
  })
}

pub async fn delete_organization(deletion_mode: DeletionMode, id: Uuid) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Organization::delete_by_id(id).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("organization"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let organization = Organization::find_by_id(id).one(db).await?;
      if organization.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("organization"))).into());
      }

      let mut organization: organization::ActiveModel = organization.unwrap().into();
      organization.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      organization.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_organization(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<OrganizationResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let organizations = Organization::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = organizations
        .into_iter()
        .map(|organization: organization::Model| OrganizationResponse::from(organization))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let organizations = Organization::find().all(db).await?;
      let res = organizations
        .into_iter()
        .map(|organization: organization::Model| OrganizationResponse::from(organization))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
