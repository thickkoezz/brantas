use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Organization, organization};
use crate::dtos::organization::OrganizationResponse;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_organization(deletion_mode: DeletionMode, id: Uuid) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Organization::delete_by_id(id).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("organization"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let organization = Organization::find_by_id(id).one(db).await?;
      if organization.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("organization"))).into());
      }

      let mut organization: organization::ActiveModel = organization.unwrap().into();
      organization.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      organization.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_organization(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<OrganizationResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let organizations = Organization::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = organizations.into_iter()
        .map(|organization: organization::Model| OrganizationResponse::from(organization))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let organizations = Organization::find().all(db).await?;
      let res = organizations.into_iter()
        .map(|organization: organization::Model| OrganizationResponse::from(organization))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
