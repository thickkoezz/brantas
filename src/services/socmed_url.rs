use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::socmed_url::SocmedUrlDTO;
use crate::entities::{prelude::SocmedUrl, socmed_url};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_socmed_url(req: SocmedUrlDTO) -> AppResult<SocmedUrlDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = socmed_url::ActiveModel {
    socmed_url: Set(req.socmed_url),
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
  };
  let socmed_url = SocmedUrl::insert(model.clone()).exec(db).await?;
  Ok(SocmedUrlDTO {
    socmed_url: socmed_url.last_insert_id.0,
    owner_id: socmed_url.last_insert_id.1,
    ..SocmedUrlDTO::from(model)
  })
}

pub async fn delete_socmed_url(
  deletion_mode: DeletionMode,
  socmed_url: String,
  owner_id: Uuid,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = SocmedUrl::delete_by_id((socmed_url, owner_id))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("socmed_url"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let socmed_url = SocmedUrl::find_by_id((socmed_url, owner_id))
        .one(db)
        .await?;
      if socmed_url.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("socmed_url"))).into());
      }

      let mut socmed_url: socmed_url::ActiveModel = socmed_url.unwrap().into();
      socmed_url.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      socmed_url.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_socmed_url(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<SocmedUrlDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let socmed_urls = SocmedUrl::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = socmed_urls
        .into_iter()
        .map(|socmed_url: socmed_url::Model| SocmedUrlDTO::from(socmed_url))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let socmed_urls = SocmedUrl::find().all(db).await?;
      let res = socmed_urls
        .into_iter()
        .map(|socmed_url: socmed_url::Model| SocmedUrlDTO::from(socmed_url))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
