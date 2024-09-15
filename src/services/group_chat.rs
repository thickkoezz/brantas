use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::group_chat::{GroupChatAddRequest, GroupChatResponse};
use crate::entities::{group_chat, prelude::GroupChat};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_group_chat(req: GroupChatAddRequest) -> AppResult<GroupChatResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = group_chat::ActiveModel {
    sender_id: Set(req.sender_id),
    group_creator_id: Set(req.group_creator_id),
    group_created_at: Set(req.group_created_at),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    content: Set(req.content),
    updated_at: Set(None),
    deleted_at: Set(None),
    replied_sender_id: Set(req.replied_sender_id),
    replied_created_at: Set(req.replied_created_at),
    forwarded_sender_id: Set(req.forwarded_sender_id),
    forwarded_group_creator_id: Set(req.forwarded_group_creator_id),
    forwarded_group_created_at: Set(req.forwarded_group_created_at),
    forwarded_created_at: Set(req.forwarded_created_at),
    forwarded_receiver_id: Set(req.forwarded_receiver_id),
    is_pinned: Set(req.is_pinned),
    pin_expired_at: Set(req.pin_expired_at),
  };
  let group_chat = GroupChat::insert(model.clone()).exec(db).await?;
  Ok(GroupChatResponse {
    sender_id: group_chat.last_insert_id.0,
    group_creator_id: group_chat.last_insert_id.1,
    group_created_at: group_chat.last_insert_id.2,
    created_at: group_chat.last_insert_id.3,
    ..GroupChatResponse::from(model)
  })
}

pub async fn delete_group_chat(
  deletion_mode: DeletionMode,
  sender_id: Uuid,
  group_creator_id: Uuid,
  group_created_at: DateTimeWithTimeZone,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result =
        GroupChat::delete_by_id((sender_id, group_creator_id, group_created_at, created_at))
          .exec(db)
          .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("chat"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let group_chat =
        GroupChat::find_by_id((sender_id, group_creator_id, group_created_at, created_at))
          .one(db)
          .await?;
      if group_chat.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("chat"))).into());
      }

      let mut group_chat: group_chat::ActiveModel = group_chat.unwrap().into();
      group_chat.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      group_chat.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_group_chat(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<GroupChatResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let group_chats = GroupChat::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = group_chats
        .into_iter()
        .map(|group_chat: group_chat::Model| GroupChatResponse::from(group_chat))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let group_chats = GroupChat::find().all(db).await?;
      let res = group_chats
        .into_iter()
        .map(|group_chat: group_chat::Model| GroupChatResponse::from(group_chat))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
