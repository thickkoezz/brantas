use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::document::DocumentDTO;
use crate::entities::{document, prelude::Document};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_document(req: DocumentDTO) -> AppResult<DocumentDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = document::ActiveModel {
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    document: Set(req.document),
    size: Set(req.size),
    updated_at: Set(None),
    deleted_at: Set(None),
    title: Set(req.title),
    caption: Set(req.caption),
    code: Set(req.code),
    slug: Set(req.slug),
    is_private: Set(req.is_private),
  };
  let document = Document::insert(model.clone()).exec(db).await?;
  Ok(DocumentDTO {
    owner_id: document.last_insert_id.0,
    created_at: document.last_insert_id.1,
    ..DocumentDTO::from(model)
  })
}

pub async fn delete_document(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Document::delete_by_id((owner_id, created_at))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("document"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let document = Document::find_by_id((owner_id, created_at)).one(db).await?;
      if document.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("document"))).into());
      }

      let mut document: document::ActiveModel = document.unwrap().into();
      document.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      document.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_document(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<DocumentDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let documents = Document::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = documents
        .into_iter()
        .map(|document: document::Model| DocumentDTO::from(document))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let documents = Document::find().all(db).await?;
      let res = documents
        .into_iter()
        .map(|document: document::Model| DocumentDTO::from(document))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
