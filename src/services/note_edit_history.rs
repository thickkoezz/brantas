use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::note_edit_history::NoteEditHistoryDTO;
use crate::entities::{note_edit_history, prelude::NoteEditHistory};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_note_edit_history(req: NoteEditHistoryDTO) -> AppResult<NoteEditHistoryDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = note_edit_history::ActiveModel {
    note_owner_id: Set(req.note_owner_id),
    note_created_at: Set(req.note_created_at),
    editor_id: Set(req.editor_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    title: Set(req.title),
    content: Set(req.content),
    deleted_at: Set(None),
  };
  let note_edit_history = NoteEditHistory::insert(model.clone()).exec(db).await?;
  Ok(NoteEditHistoryDTO {
    note_owner_id: note_edit_history.last_insert_id.0,
    note_created_at: note_edit_history.last_insert_id.1,
    editor_id: note_edit_history.last_insert_id.2,
    created_at: note_edit_history.last_insert_id.3,
    ..NoteEditHistoryDTO::from(model)
  })
}

pub async fn delete_note_edit_history(
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
      let result =
        NoteEditHistory::delete_by_id((note_owner_id, note_created_at, editor_id, created_at))
          .exec(db)
          .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("history"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let note_edit_history =
        NoteEditHistory::find_by_id((note_owner_id, note_created_at, editor_id, created_at))
          .one(db)
          .await?;
      if note_edit_history.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("history"))).into());
      }

      let mut note_edit_history: note_edit_history::ActiveModel = note_edit_history.unwrap().into();
      note_edit_history.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      note_edit_history.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_note_edit_history(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<NoteEditHistoryDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let note_edit_histories = NoteEditHistory::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = note_edit_histories
        .into_iter()
        .map(|note_edit_history: note_edit_history::Model| {
          NoteEditHistoryDTO::from(note_edit_history)
        })
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let note_edit_histories = NoteEditHistory::find().all(db).await?;
      let res = note_edit_histories
        .into_iter()
        .map(|note_edit_history: note_edit_history::Model| {
          NoteEditHistoryDTO::from(note_edit_history)
        })
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
