use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::document_comment::{DocumentCommentAddRequest, DocumentCommentResponse};
use crate::entities::{document_comment, prelude::DocumentComment};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_document_comment(
  req: DocumentCommentAddRequest,
) -> AppResult<DocumentCommentResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = document_comment::ActiveModel {
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    commented_document_owner_id: Set(req.commented_document_owner_id),
    commented_document_created_at: Set(req.commented_document_created_at),
    updated_at: Set(None),
    deleted_at: Set(None),
    content: Set(req.content),
    reaction_count: Set(0),
  };
  let document_comment = DocumentComment::insert(model.clone()).exec(db).await?;
  Ok(DocumentCommentResponse {
    owner_id: document_comment.last_insert_id.0,
    created_at: document_comment.last_insert_id.1,
    ..DocumentCommentResponse::from(model)
  })
}

pub async fn delete_document_comment(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = DocumentComment::delete_by_id((owner_id, created_at))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("comment"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let document_comment = DocumentComment::find_by_id((owner_id, created_at))
        .one(db)
        .await?;
      if document_comment.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("comment"))).into());
      }

      let mut document_comment: document_comment::ActiveModel = document_comment.unwrap().into();
      document_comment.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      document_comment.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_document_comment(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<DocumentCommentResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let document_comments = DocumentComment::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = document_comments
        .into_iter()
        .map(|document_comment: document_comment::Model| {
          DocumentCommentResponse::from(document_comment)
        })
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let document_comments = DocumentComment::find().all(db).await?;
      let res = document_comments
        .into_iter()
        .map(|document_comment: document_comment::Model| {
          DocumentCommentResponse::from(document_comment)
        })
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
