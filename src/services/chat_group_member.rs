use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::chat_group_member::ChatGroupMemberDTO;
use crate::entities::{chat_group_member, prelude::ChatGroupMember};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_chat_group_member(req: ChatGroupMemberDTO) -> AppResult<ChatGroupMemberDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = chat_group_member::ActiveModel {
    group_creator_id: Set(req.group_creator_id),
    group_created_at: Set(req.group_created_at),
    member_id: Set(req.member_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    deleted_at: Set(None),
  };
  let chat_group_member = ChatGroupMember::insert(model.clone()).exec(db).await?;
  Ok(ChatGroupMemberDTO {
    group_creator_id: chat_group_member.last_insert_id.0,
    group_created_at: chat_group_member.last_insert_id.1,
    member_id: chat_group_member.last_insert_id.2,
    ..ChatGroupMemberDTO::from(model)
  })
}

pub async fn delete_chat_group_member(
  deletion_mode: DeletionMode,
  group_creator_id: Uuid,
  group_created_at: DateTimeWithTimeZone,
  member_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = ChatGroupMember::delete_by_id((group_creator_id, group_created_at, member_id))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("member"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let chat_group_member =
        ChatGroupMember::find_by_id((group_creator_id, group_created_at, member_id))
          .one(db)
          .await?;
      if chat_group_member.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("member"))).into());
      }

      let mut chat_group_member: chat_group_member::ActiveModel = chat_group_member.unwrap().into();
      chat_group_member.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      chat_group_member.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_chat_group_member(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<ChatGroupMemberDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let chat_group_members = ChatGroupMember::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = chat_group_members
        .into_iter()
        .map(|chat_group_member: chat_group_member::Model| {
          ChatGroupMemberDTO::from(chat_group_member)
        })
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let chat_group_members = ChatGroupMember::find().all(db).await?;
      let res = chat_group_members
        .into_iter()
        .map(|chat_group_member: chat_group_member::Model| {
          ChatGroupMemberDTO::from(chat_group_member)
        })
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
