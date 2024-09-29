use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::photo::PhotoDTO;
use crate::entities::{photo, prelude::Photo};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_photo(req: PhotoDTO) -> AppResult<PhotoDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = photo::ActiveModel {
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    photo: Set(req.photo),
    size: Set(req.size),
    updated_at: Set(None),
    deleted_at: Set(None),
    title: Set(req.title),
    caption: Set(req.caption),
    code: Set(req.code),
    slug: Set(req.slug),
    is_private: Set(req.is_private),
  };
  let photo = Photo::insert(model.clone()).exec(db).await?;
  Ok(PhotoDTO {
    owner_id: photo.last_insert_id.0,
    created_at: photo.last_insert_id.1,
    ..PhotoDTO::from(model)
  })
}

pub async fn delete_photo(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Photo::delete_by_id((owner_id, created_at)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("photo"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let photo = Photo::find_by_id((owner_id, created_at)).one(db).await?;
      if photo.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("photo"))).into());
      }

      let mut photo: photo::ActiveModel = photo.unwrap().into();
      photo.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      photo.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_photo(paginator_option: Option<PaginatorOption>) -> AppResult<Vec<PhotoDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let photos = Photo::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = photos
        .into_iter()
        .map(|photo: photo::Model| PhotoDTO::from(photo))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let photos = Photo::find().all(db).await?;
      let res = photos
        .into_iter()
        .map(|photo: photo::Model| PhotoDTO::from(photo))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
