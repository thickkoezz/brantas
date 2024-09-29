use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::link::LinkDTO;
use crate::entities::{link, prelude::Link};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_link(req: LinkDTO) -> AppResult<LinkDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = link::ActiveModel {
    id: Set(Uuid::new_v4()),
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
    link_url: Set(req.link_url),
    hashtag: Set(req.hashtag),
    use_count: Set(0),
  };
  let link = Link::insert(model.clone()).exec(db).await?;
  Ok(LinkDTO {
    id: link.last_insert_id,
    ..LinkDTO::from(model)
  })
}

pub async fn delete_link(deletion_mode: DeletionMode, id: Uuid) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Link::delete_by_id(id).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("link"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let link = Link::find_by_id(id).one(db).await?;
      if link.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("link"))).into());
      }

      let mut link: link::ActiveModel = link.unwrap().into();
      link.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      link.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_link(paginator_option: Option<PaginatorOption>) -> AppResult<Vec<LinkDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let links = Link::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = links
        .into_iter()
        .map(|link: link::Model| LinkDTO::from(link))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let links = Link::find().all(db).await?;
      let res = links
        .into_iter()
        .map(|link: link::Model| LinkDTO::from(link))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
