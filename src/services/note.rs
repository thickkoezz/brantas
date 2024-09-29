use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::note::NoteDTO;
use crate::entities::{note, prelude::Note};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_note(req: NoteDTO) -> AppResult<NoteDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = note::ActiveModel {
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
    title: Set(req.title),
    content: Set(req.content),
  };
  let note = Note::insert(model.clone()).exec(db).await?;
  Ok(NoteDTO {
    owner_id: note.last_insert_id.0,
    created_at: note.last_insert_id.1,
    ..NoteDTO::from(model)
  })
}

pub async fn delete_note(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Note::delete_by_id((owner_id, created_at)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("note"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let note = Note::find_by_id((owner_id, created_at)).one(db).await?;
      if note.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("note"))).into());
      }

      let mut note: note::ActiveModel = note.unwrap().into();
      note.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      note.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_note(paginator_option: Option<PaginatorOption>) -> AppResult<Vec<NoteDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let notes = Note::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = notes
        .into_iter()
        .map(|note: note::Model| NoteDTO::from(note))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let notes = Note::find().all(db).await?;
      let res = notes
        .into_iter()
        .map(|note: note::Model| NoteDTO::from(note))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
