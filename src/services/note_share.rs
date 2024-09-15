use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::note_share::{NoteShareAddRequest, NoteShareResponse};
use crate::entities::{note_share, prelude::NoteShare};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_note_share(req: NoteShareAddRequest) -> AppResult<NoteShareResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = note_share::ActiveModel {
    note_owner_id: Set(req.note_owner_id),
    note_created_at: Set(req.note_created_at),
    editor_id: Set(req.editor_id),
    can_edit: Set(req.can_edit),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    deleted_at: Set(None),
  };
  let note_share = NoteShare::insert(model.clone()).exec(db).await?;
  Ok(NoteShareResponse {
    note_owner_id: note_share.last_insert_id.0,
    note_created_at: note_share.last_insert_id.1,
    editor_id: note_share.last_insert_id.2,
    created_at: note_share.last_insert_id.3,
    ..NoteShareResponse::from(model)
  })
}

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
      let result = NoteShare::delete_by_id((note_owner_id, note_created_at, editor_id, created_at))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("user"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let note_share =
        NoteShare::find_by_id((note_owner_id, note_created_at, editor_id, created_at))
          .one(db)
          .await?;
      if note_share.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user"))).into());
      }

      let mut note_share: note_share::ActiveModel = note_share.unwrap().into();
      note_share.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      note_share.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_note_share(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<NoteShareResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let note_shares = NoteShare::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = note_shares
        .into_iter()
        .map(|note_share: note_share::Model| NoteShareResponse::from(note_share))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let note_shares = NoteShare::find().all(db).await?;
      let res = note_shares
        .into_iter()
        .map(|note_share: note_share::Model| NoteShareResponse::from(note_share))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
