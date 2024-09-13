use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::DirectChat, direct_chat};
use crate::dtos::direct_chat::DirectChatResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_direct_chat(
  deletion_mode: DeletionMode,
  sender_id: Uuid,
  receiver_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = DirectChat::delete_by_id((
        sender_id, receiver_id, created_at,
      )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("chat"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let direct_chat = DirectChat::find_by_id((
        sender_id, receiver_id, created_at,
      )).one(db).await?;
      if direct_chat.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("chat"))).into());
      }

      let mut direct_chat: direct_chat::ActiveModel = direct_chat.unwrap().into();
      direct_chat.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      direct_chat.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_direct_chat(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<DirectChatResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let direct_chats = DirectChat::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = direct_chats.into_iter()
        .map(|direct_chat: direct_chat::Model| DirectChatResponse::from(direct_chat))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let direct_chats = DirectChat::find().all(db).await?;
      let res = direct_chats.into_iter()
        .map(|direct_chat: direct_chat::Model| DirectChatResponse::from(direct_chat))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
