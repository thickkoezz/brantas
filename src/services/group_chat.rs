use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::GroupChat, group_chat};
use crate::dtos::group_chat::GroupChatResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_group_chat(
  deletion_mode: DeletionMode,
  sender_id: Uuid,
  group_creator_id: Uuid,
  group_created_at: DateTimeWithTimeZone,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = GroupChat::delete_by_id((
        sender_id, group_creator_id, group_created_at, created_at,
      )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("chat"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let group_chat = GroupChat::find_by_id((
        sender_id, group_creator_id, group_created_at, created_at,
      )).one(db).await?;
      if group_chat.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("chat"))).into());
      }

      let mut group_chat: group_chat::ActiveModel = group_chat.unwrap().into();
      group_chat.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      group_chat.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_group_chat(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<GroupChatResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let group_chats = GroupChat::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = group_chats.into_iter()
        .map(|group_chat: group_chat::Model| GroupChatResponse::from(group_chat))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let group_chats = GroupChat::find().all(db).await?;
      let res = group_chats.into_iter()
        .map(|group_chat: group_chat::Model| GroupChatResponse::from(group_chat))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
