use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::NoteShare;
use crate::entities::note_share;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_note_share(
  deletion_mode: DeletionMode,
  note_owner_id: Uuid,
  note_created_at: DateTimeWithTimeZone,
  editor_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = NoteShare::delete_by_id((
        note_owner_id,
        note_created_at,
        editor_id,
        created_at,
      ))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("user"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let note_share = NoteShare::find_by_id((
        note_owner_id,
        note_created_at,
        editor_id,
        created_at,
      ))
        .one(db)
        .await?;
      if note_share.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user"))).into());
      }

      let mut note_share: note_share::ActiveModel = note_share.unwrap().into();
      note_share.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      note_share.update(db).await?;
      Ok(())
    },
  }
}
