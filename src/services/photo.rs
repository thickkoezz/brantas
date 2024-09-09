use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Photo;
use crate::entities::photo;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_photo(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
  photo: String,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = Photo::delete_by_id((
        owner_id,
        created_at,
        photo,
      ))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("photo"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let photo = Photo::find_by_id((
        owner_id,
        created_at,
        photo,
      ))
        .one(db)
        .await?;
      if photo.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("photo"))).into());
      }

      let mut photo: photo::ActiveModel = photo.unwrap().into();
      photo.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      photo.update(db).await?;
      Ok(())
    },
  }
}
