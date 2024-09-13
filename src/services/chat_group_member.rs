use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::ChatGroupMember, chat_group_member};
use crate::dtos::chat_group_member::ChatGroupMemberResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_chat_group_member(
  deletion_mode: DeletionMode,
  group_creator_id: Uuid,
  group_created_at: DateTimeWithTimeZone,
  member_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result =
        ChatGroupMember::delete_by_id((
          group_creator_id, group_created_at, member_id, created_at,
        )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("member"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let chat_group_member = ChatGroupMember::find_by_id((
        group_creator_id, group_created_at, member_id, created_at,
      )).one(db).await?;
      if chat_group_member.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("member"))).into());
      }

      let mut chat_group_member: chat_group_member::ActiveModel = chat_group_member.unwrap().into();
      chat_group_member.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      chat_group_member.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_chat_group_member(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<ChatGroupMemberResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let chat_group_members = ChatGroupMember::find()
        .paginate(db, paginator_option.page_size).fetch_page(paginator_option.page).await?;
      let res = chat_group_members.into_iter()
        .map(|chat_group_member: chat_group_member::Model|
          ChatGroupMemberResponse::from(chat_group_member))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let chat_group_members = ChatGroupMember::find().all(db).await?;
      let res = chat_group_members.into_iter()
        .map(|chat_group_member: chat_group_member::Model|
          ChatGroupMemberResponse::from(chat_group_member))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
