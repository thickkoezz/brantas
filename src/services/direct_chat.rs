use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::direct_chat::{DirectChatAddRequest, DirectChatResponse};
use crate::entities::{direct_chat, prelude::DirectChat};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_direct_chat(req: DirectChatAddRequest) -> AppResult<DirectChatResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = direct_chat::ActiveModel {
    sender_id: Set(req.sender_id),
    receiver_id: Set(req.receiver_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    content: Set(req.content),
    updated_at: Set(None),
    deleted_at: Set(None),
    replied_sender_id: Set(req.replied_sender_id),
    replied_receiver_id: Set(req.replied_receiver_id),
    replied_created_at: Set(req.replied_created_at),
    forwarded_sender_id: Set(req.forwarded_sender_id),
    forwarded_receiver_id: Set(req.forwarded_receiver_id),
    forwarded_created_at: Set(req.forwarded_created_at),
    forwarded_group_creator_id: Set(req.forwarded_group_creator_id),
    forwarded_group_created_at: Set(req.forwarded_group_created_at),
    is_pinned: Set(false),
    pin_expired_at: Set(None),
  };
  let direct_chat = DirectChat::insert(model.clone()).exec(db).await?;
  Ok(DirectChatResponse {
    sender_id: direct_chat.last_insert_id.0,
    receiver_id: direct_chat.last_insert_id.1,
    created_at: direct_chat.last_insert_id.2,
    ..DirectChatResponse::from(model)
  })
}

pub async fn delete_direct_chat(
  deletion_mode: DeletionMode,
  sender_id: Uuid,
  receiver_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = DirectChat::delete_by_id((sender_id, receiver_id, created_at))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("chat"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let direct_chat = DirectChat::find_by_id((sender_id, receiver_id, created_at))
        .one(db)
        .await?;
      if direct_chat.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("chat"))).into());
      }

      let mut direct_chat: direct_chat::ActiveModel = direct_chat.unwrap().into();
      direct_chat.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      direct_chat.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_direct_chat(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<DirectChatResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let direct_chats = DirectChat::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = direct_chats
        .into_iter()
        .map(|direct_chat: direct_chat::Model| DirectChatResponse::from(direct_chat))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let direct_chats = DirectChat::find().all(db).await?;
      let res = direct_chats
        .into_iter()
        .map(|direct_chat: direct_chat::Model| DirectChatResponse::from(direct_chat))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
