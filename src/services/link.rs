use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Link, link};
use crate::dtos::link::LinkResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_link(deletion_mode: DeletionMode, id: Uuid) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Link::delete_by_id(id).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("link"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let link = Link::find_by_id(id).one(db).await?;
      if link.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("link"))).into());
      }

      let mut link: link::ActiveModel = link.unwrap().into();
      link.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      link.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_link(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<LinkResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let links = Link::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = links.into_iter().map(|link: link::Model| LinkResponse::from(link))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let links = Link::find().all(db).await?;
      let res = links.into_iter().map(|link: link::Model| LinkResponse::from(link))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
