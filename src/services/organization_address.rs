use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::organization_address::{
  OrganizationAddressAddRequest, OrganizationAddressResponse,
};
use crate::entities::{organization_address, prelude::OrganizationAddress};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_organization_address(
  req: OrganizationAddressAddRequest,
) -> AppResult<OrganizationAddressResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = organization_address::ActiveModel {
    organization_id: Set(req.organization_id),
    city_id: Set(req.city_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
    description: Set(req.description),
  };
  let organization_address = OrganizationAddress::insert(model.clone()).exec(db).await?;
  Ok(OrganizationAddressResponse {
    organization_id: organization_address.last_insert_id.0,
    city_id: organization_address.last_insert_id.1,
    ..OrganizationAddressResponse::from(model)
  })
}

pub async fn delete_organization_address(
  deletion_mode: DeletionMode,
  organization_id: Uuid,
  city_id: i32,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = OrganizationAddress::delete_by_id((organization_id, city_id))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("address"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let organization_address = OrganizationAddress::find_by_id((organization_id, city_id))
        .one(db)
        .await?;
      if organization_address.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("address"))).into());
      }

      let mut organization_address: organization_address::ActiveModel =
        organization_address.unwrap().into();
      organization_address.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      organization_address.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_organization_address(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<OrganizationAddressResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let organization_address = OrganizationAddress::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = organization_address
        .into_iter()
        .map(|organization_address: organization_address::Model| {
          OrganizationAddressResponse::from(organization_address)
        })
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let organization_address = OrganizationAddress::find().all(db).await?;
      let res = organization_address
        .into_iter()
        .map(|organization_address: organization_address::Model| {
          OrganizationAddressResponse::from(organization_address)
        })
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
